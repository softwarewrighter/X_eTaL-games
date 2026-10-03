//! A game in the page, exactly as X_eTaL runs it: its terminal program
//! (`play.xtl`) run with everything typed so far as its keyboard (the
//! page is a terminal that replays the session from the start on every
//! line typed; the same seed makes the replay the same game), and its
//! scripted program run as a notebook, each statement then its output,
//! as `just show` lays it out. Pictures are only those the program
//! shows (`[]S_HOW`, SVG drawn by X_eTaL).

use std::collections::{BTreeMap, VecDeque};
use std::io::Write;
use std::sync::{Arc, Mutex};

use xetal_store::Store;

/// A line of the transcript: printed by the program, typed, or a
/// picture the program showed (an SVG document drawn by X_eTaL).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Line {
    Out(String),
    In(String),
    Picture(String),
}

/// A session replayed: the transcript, whether the program is waiting
/// for the next line (it asked and nothing was left to read), and the
/// X_eTaL error if it stopped on one. Neither waiting nor an error:
/// the game is over.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transcript {
    pub lines: Vec<Line>,
    pub waiting: bool,
    pub error: Option<String>,
}

#[derive(Default)]
struct Shared {
    pending: Vec<u8>,
    lines: Vec<Line>,
}

impl Shared {
    fn flush(&mut self) {
        let text = String::from_utf8_lossy(&std::mem::take(&mut self.pending)).into_owned();
        let mut parts: Vec<&str> = text.split('\n').collect();
        // A trailing piece without a newline is kept for later.
        let rest = parts.pop().unwrap_or("").to_string();
        self.lines.extend(parts.into_iter().map(|l| Line::Out(l.to_string())));
        self.pending = rest.into_bytes();
    }
}

/// The store a session runs on: its libraries, the lines typed, and
/// the transcript the output and the reads go into, in order.
struct Session {
    files: BTreeMap<String, String>,
    typed: Mutex<VecDeque<String>>,
    shared: Arc<Mutex<Shared>>,
}

impl Store for Session {
    fn get(&self, path: &str) -> Result<String, String> {
        self.files.get(path).cloned().ok_or_else(|| format!("{path}: no such file"))
    }

    fn put(&self, _path: &str, _text: &str) -> Result<(), String> {
        Err("a game page keeps no files".into())
    }

    fn line(&self) -> Result<String, String> {
        let next = self.typed.lock().map_err(|_| "the store is unusable".to_string())?.pop_front();
        let line = next.ok_or_else(|| NO_INPUT.to_string())?;
        if let Ok(mut s) = self.shared.lock() {
            s.flush();
            s.lines.push(Line::In(line.clone()));
        }
        Ok(line)
    }

    fn show(&self, svg: &str) -> Result<(), String> {
        if let Ok(mut s) = self.shared.lock() {
            s.flush();
            s.lines.push(Line::Picture(svg.to_string()));
        }
        Ok(())
    }
}

const NO_INPUT: &str = "no more input";

struct Out(Arc<Mutex<Shared>>);

impl Write for Out {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if let Ok(mut s) = self.0.lock() {
            s.pending.extend_from_slice(buf);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// Runs take turns: the store is global, so tests may run in parallel.
static TURN: Mutex<()> = Mutex::new(());

/// Install a store holding `libraries` and `typed`, writing into `shared`.
fn install(libraries: &[(&str, &str)], typed: &[String], shared: &Arc<Mutex<Shared>>) {
    let store = Session {
        files: libraries.iter().map(|(n, t)| (format!("{n}.xtl"), t.to_string())).collect(),
        typed: Mutex::new(typed.iter().cloned().collect()),
        shared: shared.clone(),
    };
    xetal_store::install(Arc::new(store));
}

/// What was printed since the last take, as lines (an unfinished last
/// line ended).
fn take(shared: &Arc<Mutex<Shared>>) -> Vec<Line> {
    let mut s = shared.lock().unwrap_or_else(|e| e.into_inner());
    s.flush();
    if !s.pending.is_empty() {
        s.pending.push(b'\n');
        s.flush();
    }
    std::mem::take(&mut s.lines)
}

/// Run `src` with `libraries` (name, text) where `u_se<` finds them,
/// `typed` as the keyboard, rolling from `seed`.
pub fn session(libraries: &[(&str, &str)], src: &str, typed: &[String], seed: u64) -> Transcript {
    let _turn = TURN.lock().unwrap_or_else(|e| e.into_inner());
    let shared = Arc::new(Mutex::new(Shared::default()));
    install(libraries, typed, &shared);
    let run = xetal_play::run_to(src, seed, &mut Out(shared.clone()));
    let lines = take(&shared);
    let err = run.err.trim().to_string();
    let waiting = err.contains(NO_INPUT) && err.contains("[]R_EAD");
    Transcript { lines, waiting, error: (!err.is_empty() && !waiting).then_some(err) }
}

/// One statement of a notebook (with the comments above it) and what it
/// printed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    pub source: String,
    pub output: Vec<Line>,
}

/// A program run as a notebook: its cells, and the X_eTaL error if it
/// stopped on one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Notebook {
    pub cells: Vec<Cell>,
    pub error: Option<String>,
}

/// Run `src` as a notebook (as `just show` does) with `libraries`,
/// rolling from `seed`, nothing typed.
pub fn notebook(libraries: &[(&str, &str)], src: &str, seed: u64) -> Notebook {
    let _turn = TURN.lock().unwrap_or_else(|e| e.into_inner());
    let shared = Arc::new(Mutex::new(Shared::default()));
    install(libraries, &[], &shared);
    let cells: Arc<Mutex<Vec<Cell>>> = Arc::new(Mutex::new(Vec::new()));
    let (c, sh) = (cells.clone(), shared.clone());
    let mut start = move |source: &str| {
        let printed = take(&sh);
        let mut cells = c.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(last) = cells.last_mut() {
            last.output.extend(printed);
        }
        cells.push(Cell { source: source.to_string(), output: vec![] });
    };
    let run = xetal_play::notebook_to(src, seed, None, &mut start, &mut Out(shared.clone()));
    let printed = take(&shared);
    let mut cells = std::mem::take(&mut *cells.lock().unwrap_or_else(|e| e.into_inner()));
    if let Some(last) = cells.last_mut() {
        last.output.extend(printed);
    }
    let err = run.err.trim().to_string();
    Notebook { cells, error: (!err.is_empty()).then_some(err) }
}
