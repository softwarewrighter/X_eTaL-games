//! The parts of the rules the page can highlight.

use microscope::source::{between, block, find, Range, NONE};
use yew::Html;

use crate::micro::{PLAY, RULES};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Secret,
    Answer,
    Reply,
    Possible,
}

pub const STAGES: [Stage; 4] = [Stage::Secret, Stage::Answer, Stage::Reply, Stage::Possible];

/// The part of the library (NumberGuess.xtl) defining a stage.
pub fn range(stage: Stage) -> Range {
    match stage {
        Stage::Secret => between(RULES, "l:n_ew := ", "l:n_ew := "),
        Stage::Answer => between(RULES, "l:m_ove := ", "l:m_ove := "),
        Stage::Reply => between(RULES, "l:r_eply := ", "l:r_eply := "),
        Stage::Possible => between(RULES, "l:p_ossible := ", "'r_ight t_able"),
    }
}

pub fn source(focus: Stage) -> Html {
    block(RULES, range(focus))
}

/// Where the terminal program (play.xtl) calls a stage (the mask of
/// possible secrets is the page's own question, micro::board).
pub fn call(stage: Stage) -> Range {
    match stage {
        Stage::Secret => find(PLAY, "t := g:n_ew g:top"),
        Stage::Answer => find(PLAY, "r := t g:m_ove u:a_sk n"),
        Stage::Reply => find(PLAY, "shown := p_rint! g:r_eply r"),
        Stage::Possible => NONE,
    }
}

pub fn program(focus: Stage) -> Html {
    block(PLAY, call(focus))
}
