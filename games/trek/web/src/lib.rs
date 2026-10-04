//! Star Trek in the browser, exactly as X_eTaL runs it: the shared game
//! page (shared/microscope) with this game's files, the same ones the
//! command line runs.

use microscope::page::Game;

pub fn game() -> Game {
    Game {
        title: "Star Trek",
        lede: "The COR24 BASIC Star Trek in X_eTaL: destroy every Klingon in an 8 by 8 galaxy before the stardate runs out. The galaxy is three 8 by 8 planes made in one go; a course is a path of every step at once; phasers hit every Klingon in one subtraction. Commands by number, 9 is help; courses 1 (north) to 8, clockwise.",
        play: include_str!("../../play.xtl"),
        script_name: "trek.xtl",
        script: include_str!("../../trek.xtl"),
        library_name: "StarTrek",
        library: include_str!("../../StarTrek.xtl"),
        shared: &[("Board", include_str!("../../../../lib/Board.xtl")), ("State", include_str!("../../../../lib/State.xtl")), ("Text", include_str!("../../../../lib/Text.xtl")), ("Play", include_str!("../../../../lib/Play.xtl"))],
        toml: include_str!("../../game.toml"),
    }
}
