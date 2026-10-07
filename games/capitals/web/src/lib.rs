//! Capitals in the browser, exactly as X_eTaL runs it: the shared game
//! page (shared/microscope) with this game's files and data, the same
//! ones the command line runs.

use microscope::page::Game;

pub fn game() -> Game {
    Game {
        title: "Capitals",
        lede: "Name the capital at the dot: four choices, the place and its three nearest. X_eTaL reads the places and the coastlines from TOML, projects every point onto the map at once, draws it, and finds the nearest capitals by computing every distance at once.",
        play: include_str!("../../play.xtl"),
        script_name: "capitals.xtl",
        script: include_str!("../../capitals.xtl"),
        library_name: "Atlas",
        library: include_str!("../../Atlas.xtl"),
        shared: &[
            ("Play", include_str!("../../../../lib/Play.xtl")),
            ("places.toml", include_str!("../../places.toml")),
            ("assets/cache/world.toml", include_str!("../../assets/cache/world.toml")),
        ],
        toml: include_str!("../../game.toml"),
    }
}
