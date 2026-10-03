//! Trek adventure in the browser, exactly as X_eTaL runs it: the shared
//! game page (shared/microscope) with this game's files, the same ones
//! the command line runs.

use microscope::page::Game;

pub fn game() -> Game {
    Game {
        title: "Trek adventure",
        lede: "Star Trek: Decaying Orbit, the COR24 BASIC text adventure, in X_eTaL: you wake alone on the Enterprise's bridge and must restore engine power before the orbit decays. The whole game is tables: every line of text is a row with a key and a condition, a room is one mask over them, the exits a 9 by 9 table. Pick commands by number; 8 is help.",
        play: include_str!("../../play.xtl"),
        script_name: "trek-adventure.xtl",
        script: include_str!("../../trek-adventure.xtl"),
        library_name: "TrekAdventure",
        library: include_str!("../../TrekAdventure.xtl"),
    }
}
