//! Sudoku in the browser, exactly as X_eTaL runs it: the shared game
//! page (shared/microscope) with this game's files, the same ones the
//! command line runs.

use microscope::page::Game;

pub fn game() -> Game {
    Game {
        title: "Sudoku",
        lede: "Fill the grid so every row, column and box holds 1 to 9. X_eTaL keeps every cell's candidates as one 81 by 9 array, finds every single at once by summing over the 27 units, repeats until nothing changes, and searches only when that stalls.",
        play: include_str!("../../play.xtl"),
        interactive: false,
        ticks: false,
        script_name: "sudoku.xtl",
        script: include_str!("../../sudoku.xtl"),
        library_name: "SudokuGrid",
        library: include_str!("../../SudokuGrid.xtl"),
        shared: &[("Play", include_str!("../../../../lib/Play.xtl")), ("Text", include_str!("../../../../lib/Text.xtl")), ("puzzles.toml", include_str!("../../puzzles.toml"))],
        toml: include_str!("../../game.toml"),
    }
}
