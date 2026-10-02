//! The model: the terminal game (play.xtl) replayed from what was typed,
//! and what the guesses so far say, computed by the rules library
//! (NumberGuess.xtl): every candidate against every guess at once.
//! Nothing here knows about the browser, and no rule is written here.

use microscope::run::numbers;
use microscope::terminal::{session, Line, Transcript};

pub const PLAY: &str = include_str!("../../play.xtl");
pub const RULES: &str = include_str!("../../NumberGuess.xtl");
pub const TOP: usize = 100;

const LIBS: [(&str, &str); 1] = [("NumberGuess", RULES)];

/// The game so far: play.xtl with `typed` as its keyboard.
pub fn replay(typed: &[String], seed: u64) -> Transcript {
    session(&LIBS, PLAY, typed, seed)
}

/// The guesses the game took from what was typed: the first number on
/// each line it read (as play.xtl reads it: characters other than
/// digits and spaces dropped; a line without a number is asked again).
pub fn guesses(t: &Transcript) -> Vec<i64> {
    t.lines
        .iter()
        .filter_map(|l| match l {
            Line::In(s) => {
                let kept: String = s.chars().filter(|c| c.is_ascii_digit() || *c == ' ').collect();
                kept.split_whitespace().next().and_then(|w| w.parse().ok())
            }
            Line::Out(_) => None,
        })
        .collect()
}

/// What the guesses say, for the game rolled from `seed`: the answer to
/// each (-1 higher, 0 right, 1 lower) and which of 1 to 100 are still
/// possible.
#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    pub answers: Vec<i64>,
    pub possible: Vec<bool>,
}

pub fn board(gs: &[i64], seed: u64) -> Result<Board, String> {
    if gs.is_empty() {
        return Ok(Board { answers: vec![], possible: vec![true; TOP] });
    }
    let v: Vec<String> = gs.iter().map(i64::to_string).collect();
    let src = format!(
        "\"g:\" u_se< \"NumberGuess\"\nt := g:n_ew g:top\ngs := {} r_eshape {}\nt g:m_ove gs\nt g:p_ossible gs\n",
        gs.len(),
        v.join(" ")
    );
    let t = session(&LIBS, &src, &[], seed);
    if let Some(e) = t.error {
        return Err(e);
    }
    let out: Vec<&str> = t.lines.iter().filter_map(|l| match l { Line::Out(s) => Some(s.as_str()), _ => None }).collect();
    if out.len() != 2 {
        return Err(format!("expected 2 lines of output, got {}", out.len()));
    }
    let ok: Vec<i64> = numbers(out[1], TOP)?;
    Ok(Board { answers: numbers(out[0], gs.len())?, possible: ok.iter().map(|&b| b == 1).collect() })
}
