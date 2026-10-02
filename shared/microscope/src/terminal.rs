//! A text game in the page: its terminal program (`play.xtl`) run with
//! everything typed so far as its keyboard, so the page is a terminal
//! that replays the session from the start on every line typed. The
//! same seed every time makes the replay the same game.

use std::collections::{BTreeMap, VecDeque};
use std::io::Write;
use std::sync::{Arc, Mutex};

use xetal_store::Store;

/// A line of the transcript: printed by the program, or typed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Line {
    Out(String),
    In(String),
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

/// Run `src` with `libraries` (name, text) where `u_se<` finds them,
/// `typed` as the keyboard, rolling from `seed`. Sessions take turns
/// (the store is global), so tests may run them in parallel.
pub fn session(libraries: &[(&str, &str)], src: &str, typed: &[String], seed: u64) -> Transcript {
    static TURN: Mutex<()> = Mutex::new(());
    let _turn = TURN.lock().unwrap_or_else(|e| e.into_inner());
    let shared = Arc::new(Mutex::new(Shared::default()));
    let store = Session {
        files: libraries.iter().map(|(n, t)| (format!("{n}.xtl"), t.to_string())).collect(),
        typed: Mutex::new(typed.iter().cloned().collect()),
        shared: shared.clone(),
    };
    xetal_store::install(Arc::new(store));
    let run = xetal_play::run_to(src, seed, &mut Out(shared.clone()));
    let mut s = shared.lock().unwrap_or_else(|e| e.into_inner());
    s.pending.push(b'\n');
    s.flush();
    let mut lines = std::mem::take(&mut s.lines);
    if lines.last() == Some(&Line::Out(String::new())) {
        lines.pop();
    }
    let err = run.err.trim().to_string();
    let waiting = err.contains(NO_INPUT) && err.contains("[]R_EAD");
    Transcript { lines, waiting, error: (!err.is_empty() && !waiting).then_some(err) }
}
