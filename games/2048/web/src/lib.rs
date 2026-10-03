//! 2048 in the browser, exactly as X_eTaL runs it: the shared game page
//! (shared/microscope) with this game's files, the same ones the command
//! line runs.

use microscope::page::Game;

pub fn game() -> Game {
    Game {
        title: "2048",
        lede: "2048 in X_eTaL: a move left slides and merges every row at once (compress by running counts and a table, merge by places in runs of equal tiles), and the other moves turn the board with reverse and transpose. Move with a, d, w, s; q stops.",
        play: include_str!("../../play.xtl"),
        script_name: "2048.xtl",
        script: include_str!("../../2048.xtl"),
        library_name: "Twenty48",
        library: include_str!("../../Twenty48.xtl"),
        shared: &[],
    }
}
