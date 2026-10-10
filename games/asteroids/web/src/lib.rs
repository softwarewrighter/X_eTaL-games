//! Asteroids in the browser, exactly as X_eTaL runs it: the shared game
//! page (shared/microscope) with this game's files and data, the same
//! ones the command line runs.

use microscope::page::Game;

pub fn game() -> Game {
    Game {
        title: "Asteroids",
        lede: "Break every rock with the arrow keys and the space bar: big rocks break into medium ones, medium into small. The game moves with the clock: the page gives the program a tick twenty times a second and your keys, and shows X_eTaL's picture. Shots and rocks are spawned and despawned all the time in the slots a mask says are free, on an entity-component system written in X_eTaL.",
        play: include_str!("../../play.xtl"),
        interactive: true,
        ticks: true,
        script_name: "asteroids.xtl",
        script: include_str!("../../asteroids.xtl"),
        library_name: "Rocks",
        library: include_str!("../../Rocks.xtl"),
        shared: &[
            ("Ecs", include_str!("../../../../lib/Ecs.xtl")),
            ("Ecs.xtlm", include_str!("../../../../lib/Ecs.xtlm")),
            ("Text", include_str!("../../../../lib/Text.xtl")),
            ("Svg", include_str!("../../../../lib/Svg.xtl")),
            ("asteroids.toml", include_str!("../../asteroids.toml")),
        ],
        toml: include_str!("../../game.toml"),
    }
}
