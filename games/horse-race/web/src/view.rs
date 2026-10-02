//! The stages of a round and the part of the rules computing each.

use microscope::source::{between, block, Range};
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

pub fn range(stage: Stage) -> Range {
    match stage {
        Stage::Roll => between(RULES, "l:r_oll := ", "l:r_oll := "),
        Stage::Move => between(RULES, "l:m_ove := ", "l:m_ove := "),
        Stage::Finish => between(RULES, "l:s_tatus := ", "l:s_tatus := "),
        Stage::Winners => between(RULES, "l:w_inners := ", "l:w_inners := "),
        Stage::Track => between(RULES, "l:v_iew := ", "s_elect \" #:\""),
    }
}

pub fn source(focus: Stage) -> Html {
    block(RULES, range(focus))
}
