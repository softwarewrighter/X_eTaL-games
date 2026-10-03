//! Tic-tac-toe in the browser, exactly as X_eTaL runs it: the shared
//! game page (shared/microscope) with this game's files, the same ones
//! the command line runs.

use microscope::page::Game;

pub fn game() -> Game {
    Game {
        title: "Tic-tac-toe",
        lede: "Tic-tac-toe in X_eTaL: the board is a vector of 9, the eight lines a table of squares, and one selection and one reduce give every line's sum at once. You are X; the computer rates every empty square at once. The scripted game also has a minimax player.",
        play: include_str!("../../play.xtl"),
        script_name: "tic-tac-toe.xtl",
        script: include_str!("../../tic-tac-toe.xtl"),
        library_name: "TicTacToe",
        library: include_str!("../../TicTacToe.xtl"),
    }
}
