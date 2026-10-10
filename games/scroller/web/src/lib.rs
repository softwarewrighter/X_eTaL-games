//! The side-scroller in the browser, exactly as X_eTaL runs it: the
//! shared game page (shared/microscope) with this game's files and data,
//! the same ones the command line runs.

use microscope::page::Game;

pub fn game() -> Game {
    Game {
        title: "Side-scroller",
        lede: "Run and jump to the flag with the arrow keys or WASD: land on the walkers, jump the pits, pick up the coins. The game moves with the clock: the page gives the program a tick twenty times a second and your keys, and shows X_eTaL's picture. Gravity, movement and collision are each one expression over every body at once, on an entity-component system written in X_eTaL.",
        play: include_str!("../../play.xtl"),
        interactive: true,
        ticks: true,
        script_name: "scroller.xtl",
        script: include_str!("../../scroller.xtl"),
        library_name: "Jump",
        library: include_str!("../../Jump.xtl"),
        shared: &[
            ("Ecs", include_str!("../../../../lib/Ecs.xtl")),
            ("Ecs.xtlm", include_str!("../../../../lib/Ecs.xtlm")),
            ("Text", include_str!("../../../../lib/Text.xtl")),
            ("Svg", include_str!("../../../../lib/Svg.xtl")),
            ("scroller.toml", include_str!("../../scroller.toml")),
        ],
        toml: include_str!("../../game.toml"),
    }
}
