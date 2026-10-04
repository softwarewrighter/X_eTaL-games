//! Minesweeper in the browser, exactly as X_eTaL runs it: the shared
//! game page (shared/microscope) with this game's files, the same ones
//! the command line runs.

use microscope::page::Game;

pub fn game() -> Game {
    Game {
        title: "Minesweeper",
        lede: "Minesweeper in X_eTaL: every square's count of neighbouring mines at once, as Life counts neighbours (the mines rotated by every offset and summed), and opening an empty square floods by growing the opened region until it stops. Type a row and a column to open, f row column to flag.",
        play: include_str!("../../play.xtl"),
        script_name: "minesweeper.xtl",
        script: include_str!("../../minesweeper.xtl"),
        library_name: "Mines",
        library: include_str!("../../Mines.xtl"),
        shared: &[("Board", include_str!("../../../../lib/Board.xtl")), ("Play", include_str!("../../../../lib/Play.xtl"))],
        toml: include_str!("../../game.toml"),
    }
}
