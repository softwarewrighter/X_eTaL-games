//! Guess the number in the browser, exactly as X_eTaL runs it: the
//! shared game page (shared/microscope) with this game's files, the
//! same ones the command line runs.

use microscope::page::Game;

pub fn game() -> Game {
    Game {
        title: "Guess the number",
        lede: "The smallest game of the COR24 BASIC demos, in X_eTaL: I am thinking of a number from 1 to 100; you guess, I say higher or lower. The scripted game answers many guesses at once, checks every candidate against every guess, and plays all 100 games in lockstep.",
        play: include_str!("../../play.xtl"),
        script_name: "guess.xtl",
        script: include_str!("../../guess.xtl"),
        library_name: "NumberGuess",
        library: include_str!("../../NumberGuess.xtl"),
        shared: &[("Play", include_str!("../../../../lib/Play.xtl"))],
    }
}
