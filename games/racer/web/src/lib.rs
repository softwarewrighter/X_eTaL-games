//! The racer in the browser, exactly as X_eTaL runs it: the shared game
//! page (shared/microscope) with this game's files and data, the same
//! ones the command line runs.

use microscope::page::Game;

pub fn game() -> Game {
    Game {
        title: "Racer",
        lede: "Three laps against three rivals: up to go, left and right to steer, keep off the grass. The race moves with the clock: the page gives the program a tick twenty times a second and your keys, and shows X_eTaL's picture. Driving, the grass, bumping, laps and places are each one expression over every car at once, on an entity-component system written in X_eTaL.",
        play: include_str!("../../play.xtl"),
        interactive: true,
        ticks: true,
        script_name: "racer.xtl",
        script: include_str!("../../racer.xtl"),
        library_name: "Race",
        library: include_str!("../../Race.xtl"),
        shared: &[
            ("Ecs", include_str!("../../../../lib/Ecs.xtl")),
            ("Ecs.xtlm", include_str!("../../../../lib/Ecs.xtlm")),
            ("Text", include_str!("../../../../lib/Text.xtl")),
            ("Svg", include_str!("../../../../lib/Svg.xtl")),
            ("racer.toml", include_str!("../../racer.toml")),
        ],
        toml: include_str!("../../game.toml"),
    }
}
