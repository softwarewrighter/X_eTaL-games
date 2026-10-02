//! The stages of a round and the part of the rules computing each.

use microscope::source::{between, block, find, Range};
use yew::Html;

use crate::micro::RULES;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Roll,
    Move,
    Finish,
    Winners,
    Track,
}

pub const STAGES: [Stage; 5] = [Stage::Roll, Stage::Move, Stage::Finish, Stage::Winners, Stage::Track];

/// The part of the library (HorseRace.xtl) defining a stage.
pub fn range(stage: Stage) -> Range {
    match stage {
        Stage::Roll => between(RULES, "l:r_oll := ", "l:r_oll := "),
        Stage::Move => between(RULES, "l:m_ove := ", "l:m_ove := "),
        Stage::Finish => between(RULES, "l:s_tatus := ", "l:s_tatus := "),
        Stage::Winners => between(RULES, "l:w_inners := ", "l:w_inners := "),
        Stage::Track => between(RULES, "l:v_iew := ", "s_elect \" #:\""),
    }
}

/// The library, the defining part of the stage highlighted.
pub fn source(focus: Stage) -> Html {
    block(RULES, range(focus))
}

/// The call of a stage in the program the page runs (micro::program).
pub fn call(stage: Stage) -> &'static str {
    match stage {
        Stage::Roll => "r := h:r_oll p",
        Stage::Move => "q := p h:m_ove r",
        Stage::Finish => "h:s_tatus q",
        Stage::Winners => "h:w_inners q",
        Stage::Track => "h:v_iew q",
    }
}

/// The program the page runs for a round, the stage's call highlighted.
pub fn program(src: &str, focus: Stage) -> Html {
    block(src, find(src, call(focus)))
}
