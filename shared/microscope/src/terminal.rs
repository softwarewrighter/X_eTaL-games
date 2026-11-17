//! A game in the page, exactly as X_eTaL runs it: its terminal program
//! (`play.xtl`) kept running (X_eTaL's resumable run: it stops where it
//! reads a line and goes on from there when one is typed; the same seed
//! and the same lines make the same game), and its
//! scripted program run as a notebook, each statement then its output,
//! as `just show` lays it out. Pictures are only those the program
//! shows (`[]S_HOW`, SVG drawn by X_eTaL).

use std::collections::BTreeMap;
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

/// A session so far: the transcript, whether the program is waiting
/// for the next line, and the
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

/// The store a program runs on: its libraries and data files, and the
/// transcript its output and pictures go into, in order. It has no
/// keyboard of its own: a running game is fed its lines (Live), and a
/// notebook reads none.
struct Session {
    files: BTreeMap<String, String>,
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
        Err("no more input".into())
    }

    fn show(&self, svg: &str) -> Result<(), String> {
        if let Ok(mut s) = self.shared.lock() {
            s.flush();
            s.lines.push(Line::Picture(svg.to_string()));
        }
        Ok(())
    }
}

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

/// The store path of a library or data file: a library by its name
/// (`Board` is `Board.xtl`), a data file by its path, extension and all
/// (`places.toml`, `assets/cache/world.toml`), as `[]T_ABLE` reads it.
fn path(name: &str) -> String {
    let file = name.rsplit('/').next().unwrap_or(name);
    if file.contains('.') { name.to_string() } else { format!("{name}.xtl") }
}

/// A store holding `libraries` (and data files), writing into `shared`.
fn store(libraries: &[(&str, &str)], shared: &Arc<Mutex<Shared>>) -> Arc<Session> {
    Arc::new(Session { files: libraries.iter().map(|(n, t)| (path(n), t.to_string())).collect(), shared: shared.clone() })
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

/// A whole session at once: `src` started with `libraries` (name, text)
/// where `u_se<` finds them, rolling from `seed`, and given each line of
/// `typed` where it waits (Live, as the page runs it; lines typed after
/// the program ended are not read), then the end of its input as the
/// command line gives it with a file for a keyboard (Live::end): what
/// the tests compare with the command line's goldens.
pub fn session(libraries: &[(&str, &str)], src: &str, typed: &[String], seed: u64) -> Transcript {
    let mut live = Live::start(libraries, src, seed);
    for line in typed {
        live.feed(line);
    }
    live.end();
    live.transcript()
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
    xetal_store::install(store(libraries, &shared));
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

/// A game kept running (X_eTaL's resumable run,
/// `xetal_play::Interactive`): started once, it stops where it waits for
/// the next line (typed, or an event: a click, a key, a tick), and each
/// line fed to it goes on from there, so a line costs only its own work.
/// Its store (libraries, data files, the
/// transcript) is installed again before each step, since the notebook
/// installs one of its own.
pub struct Live {
    run: Option<xetal_play::Interactive>,
    store: Arc<Session>,
    shared: Arc<Mutex<Shared>>,
    lines: Vec<Line>,
    error: Option<String>,
    over: bool,
    /// Waiting for an event (`[]E_VENT`), not a typed line.
    event: bool,
    lean: bool,
}

/// Transitions per slice: a slice ends where the program waits, so this
/// only bounds a runaway loop between two waits.
const SLICE: usize = 50_000_000;

impl Live {
    /// Start `src` with `libraries`, rolling from `seed`, and run it to
    /// its first wait.
    pub fn start(libraries: &[(&str, &str)], src: &str, seed: u64) -> Live {
        let _turn = TURN.lock().unwrap_or_else(|e| e.into_inner());
        let shared = Arc::new(Mutex::new(Shared::default()));
        let store = store(libraries, &shared);
        xetal_store::install(store.clone());
        let (run, error) = match xetal_play::Interactive::start(src, seed, Box::new(Out(shared.clone()))) {
            Ok(run) => (Some(run), None),
            Err(r) => (None, Some(r.err.trim().to_string()).filter(|e| !e.is_empty())),
        };
        let mut live = Live { run, store, shared, lines: vec![], error, over: false, event: false, lean: false };
        live.go();
        live
    }

    /// For a game moved by the clock (Game::ticks): its ticks are not kept
    /// in the transcript, nor any picture but the latest, so twenty
    /// pictures a second do not pile up while it is played.
    pub fn lean(mut self) -> Live {
        self.lean = true;
        self.trim();
        self
    }

    /// The latest picture only, when lean.
    fn trim(&mut self) {
        if !self.lean {
            return;
        }
        if let Some(last) = self.lines.iter().rposition(|l| matches!(l, Line::Picture(_))) {
            let mut i = 0;
            self.lines.retain(|l| {
                i += 1;
                !matches!(l, Line::Picture(_)) || i - 1 == last
            });
        }
    }

    /// Run until the program waits, ends or stops on an error.
    fn go(&mut self) {
        let Some(run) = self.run.as_mut() else {
            self.over = true;
            self.lines.extend(take(&self.shared));
            self.trim();
            return;
        };
        xetal_store::install(self.store.clone());
        loop {
            match run.step(SLICE) {
                xetal_play::Step::Running => continue,
                xetal_play::Step::Done(r) => {
                    let e = r.err.trim().to_string();
                    self.error = (!e.is_empty()).then_some(e);
                    self.over = true;
                    self.run = None;
                    break;
                }
                step => {
                    self.event = matches!(step, xetal_play::Step::WaitingEvent);
                    break;
                }
            }
        }
        self.lines.extend(take(&self.shared));
        self.trim();
    }

    /// A line (typed, or an event, `click X Y`) for the waiting program,
    /// and its run to the next wait.
    pub fn feed(&mut self, line: &str) {
        if self.over {
            return;
        }
        let _turn = TURN.lock().unwrap_or_else(|e| e.into_inner());
        if !(self.lean && line.starts_with("tick ")) {
            self.lines.push(Line::In(line.to_string()));
        }
        if let Some(run) = self.run.as_mut() {
            run.feed(line.to_string());
        }
        self.go();
    }

    /// The end of the input, as the command line's when its keyboard is a
    /// file that has run out: a program waiting for an event is given the
    /// `end` event (and usually ends); one waiting for a typed line stays
    /// waiting. The page never ends a game's input; the tests do.
    pub fn end(&mut self) {
        if self.over || !self.event {
            return;
        }
        let _turn = TURN.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(run) = self.run.as_mut() {
            run.feed("end".to_string());
        }
        self.go();
    }

    /// The transcript so far: waiting unless the program ended.
    pub fn transcript(&self) -> Transcript {
        Transcript { lines: self.lines.clone(), waiting: !self.over, error: self.error.clone() }
    }
}
