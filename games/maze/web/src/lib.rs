//! The maze in the browser, exactly as X_eTaL runs it: the shared game
//! page (shared/microscope) with this game's files and data, the same
//! ones the command line runs.

use microscope::page::Game;

pub fn game() -> Game {
    Game {
        title: "Maze",
        lede: "Find the way out of a maze seen from inside: up to step, left and right to turn, m for a mini map of what you have seen. A new maze every game, carved by X_eTaL with whole-array operations; the corridors ahead are looked up at every depth at once and drawn by the program as SVG. The page only shows X_eTaL's picture and passes your keys to the program.",
        play: include_str!("../../play.xtl"),
        interactive: true,
        ticks: false,
        script_name: "maze.xtl",
        script: include_str!("../../maze.xtl"),
        library_name: "Walls",
        library: include_str!("../../Walls.xtl"),
        shared: &[
            ("Text", include_str!("../../../../lib/Text.xtl")),
            ("Svg", include_str!("../../../../lib/Svg.xtl")),
            ("maze.toml", include_str!("../../maze.toml")),
        ],
        toml: include_str!("../../game.toml"),
    }
}
