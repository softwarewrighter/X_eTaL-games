//! The page's state: what was typed and the game's seed. Everything
//! else is computed by X_eTaL from those (micro).

use std::rc::Rc;

use microscope::terminal::Transcript;
use yew::Reducible;

use crate::micro::{board, guesses, replay, Board, TOP};
use crate::view::Stage;

#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    pub typed: Vec<String>,
    pub seed: u64,
    pub transcript: Transcript,
    pub board: Board,
    pub focus: Stage,
    pub notice: Option<String>,
}

pub enum Action {
    Type(String),
    NewGame,
    Focus(Stage),
}

/// A seed for a new game: the clock in the browser, fixed natively.
fn clock() -> u64 {
    #[cfg(target_arch = "wasm32")]
    {
        js_sys::Date::now() as u64
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        1
    }
}

impl Model {
    pub fn new() -> Self {
        Model::with(vec![], clock(), Stage::Answer)
    }

    fn with(typed: Vec<String>, seed: u64, focus: Stage) -> Self {
        let transcript = replay(&typed, seed);
        let (board, notice) = match board(&guesses(&transcript), seed) {
            Ok(b) => (b, transcript.error.clone().map(|e| format!("X_eTaL stopped: {e}"))),
            Err(e) => (Board { answers: vec![], possible: vec![true; TOP] }, Some(format!("X_eTaL stopped: {e}"))),
        };
        Model { typed, seed, transcript, board, focus, notice }
    }

    pub fn over(&self) -> bool {
        !self.transcript.waiting
    }
}

impl Default for Model {
    fn default() -> Self {
        Model::new()
    }
}

impl Reducible for Model {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Action) -> Rc<Self> {
        let m = (*self).clone();
        Rc::new(match action {
            Action::Type(line) if m.transcript.waiting => {
                let mut typed = m.typed.clone();
                typed.push(line);
                Model::with(typed, m.seed, m.focus)
            }
            Action::Type(_) => m,
            Action::NewGame => Model { focus: m.focus, ..Model::new() },
            Action::Focus(s) => Model { focus: s, ..m },
        })
    }
}
