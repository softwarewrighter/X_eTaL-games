//! The page's state and what each control does to it. Every rule comes
//! back from X_eTaL (micro); here is only which round, which seed and
//! which horse you picked.

use std::rc::Rc;

use yew::Reducible;

use crate::micro::{round, start, Round, HORSES};
use crate::view::Stage;

#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    pub pos: Vec<i64>,
    pub rounds: u64,
    pub last: Option<Round>,
    pub track: Vec<String>,
    pub pick: Option<usize>,
    pub seed: u64,
    pub playing: bool,
    pub focus: Stage,
    pub notice: Option<String>,
}

pub enum Action {
    Tick,
    Step,
    Go,
    NewRace,
    Pick(usize),
    Focus(Stage),
}

/// A seed for a new race: the clock in the browser (a different race
/// each time), a fixed one natively (tests repeat).
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
        let (track, notice) = match start() {
            Ok(t) => (t, None),
            Err(e) => (vec![], Some(format!("X_eTaL stopped: {e}"))),
        };
        Model {
            pos: vec![0; HORSES],
            rounds: 0,
            last: None,
            track,
            pick: None,
            seed: clock(),
            playing: false,
            focus: Stage::Roll,
            notice,
        }
    }

    pub fn over(&self) -> bool {
        self.last.as_ref().is_some_and(|r| r.over)
    }

    fn advance(self) -> Self {
        if self.over() {
            return Model { playing: false, ..self };
        }
        match round(&self.pos, self.seed.wrapping_add(self.rounds)) {
            Ok(r) => Model {
                pos: r.pos.clone(),
                track: r.track.clone(),
                rounds: self.rounds + 1,
                playing: self.playing && !r.over,
                last: Some(r),
                notice: None,
                ..self
            },
            Err(e) => Model { playing: false, notice: Some(format!("X_eTaL stopped: {e}")), ..self },
        }
    }

    fn fresh(&self) -> Self {
        Model { pick: self.pick, focus: self.focus, ..Model::new() }
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
            Action::Tick => m.advance(),
            Action::Step => match m.over() {
                true => Model { playing: false, ..m.fresh() }.advance(),
                false => Model { playing: false, ..m }.advance(),
            },
            Action::Go => match m.over() || m.rounds > 0 {
                true => Model { playing: true, ..m.fresh() },
                false => Model { playing: true, ..m },
            },
            Action::NewRace => m.fresh(),
            Action::Pick(i) => Model { pick: Some(i), ..m },
            Action::Focus(s) => Model { focus: s, ..m },
        })
    }
}
