//! The model: the game's rules library (HorseRace.xtl), the same file
//! the command-line programs import, run a round at a time on the
//! positions the page keeps. Nothing here knows about the browser, and
//! no rule is written here: rolls, moves, the finish, the winners and
//! the track all come back from X_eTaL.

use microscope::run::{library, numbers, output_seeded};

/// The rules, as the command line reads them.
pub const RULES: &str = include_str!("../../HorseRace.xtl");

pub const HORSES: usize = 5;

/// After a round: the rolls, the new positions, whether the race is
/// over, which horses lead (1 where a horse is a winner, once it is
/// over), and the track as X_eTaL draws it.
#[derive(Clone, Debug, PartialEq)]
pub struct Round {
    pub rolls: Vec<i64>,
    pub pos: Vec<i64>,
    pub over: bool,
    pub winners: Vec<bool>,
    pub track: Vec<String>,
}

/// The program: one round from `pos`.
pub fn program(pos: &[i64]) -> String {
    let p: Vec<String> = pos.iter().map(i64::to_string).collect();
    format!(
        "\"h:\" u_se< \"HorseRace\"\np := {}\nr := h:r_oll p\nq := p h:m_ove r\nr\nq\nh:s_tatus q\n\
         (r_ange t_ally q) m_ember? h:w_inners q\nh:v_iew q\n",
        p.join(" ")
    )
}

/// The track before the first round, as X_eTaL draws it.
pub fn start() -> Result<Vec<String>, String> {
    library("HorseRace", RULES);
    let src = format!("\"h:\" u_se< \"HorseRace\"\nh:v_iew h:n_ew {HORSES}\n");
    output_seeded(&src, HORSES, 1)
}

/// One round from `pos`, rolling from `seed`.
pub fn round(pos: &[i64], seed: u64) -> Result<Round, String> {
    library("HorseRace", RULES);
    let out = output_seeded(&program(pos), 4 + HORSES, seed)?;
    let over = numbers::<i64>(&out[2], 1)?[0] == 1;
    let lead: Vec<i64> = numbers(&out[3], HORSES)?;
    Ok(Round {
        rolls: numbers(&out[0], HORSES)?,
        pos: numbers(&out[1], HORSES)?,
        over,
        winners: lead.iter().map(|&w| over && w == 1).collect(),
        track: out[4..].to_vec(),
    })
}
