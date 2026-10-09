//! The dungeon in the browser, exactly as X_eTaL runs it: the shared game
//! page (shared/microscope) with this game's files and data, the same
//! ones the command line runs.

use microscope::page::Game;

pub fn game() -> Game {
    Game {
        title: "Dungeon",
        lede: "Find the way out with the arrow keys or WASD: fight the monsters, pick up potions and gold. Built on an entity-component system written in X_eTaL: you, the monsters and the items are rows of one matrix, what each is comes from its components, and every rule is one expression over all of them. The page only shows X_eTaL's picture and passes your keys to the program.",
        play: include_str!("../../play.xtl"),
        interactive: true,
        ticks: false,
        script_name: "dungeon.xtl",
        script: include_str!("../../dungeon.xtl"),
        library_name: "Crawl",
        library: include_str!("../../Crawl.xtl"),
        shared: &[
            ("Ecs", include_str!("../../../../lib/Ecs.xtl")),
            ("Ecs.xtlm", include_str!("../../../../lib/Ecs.xtlm")),
            ("Text", include_str!("../../../../lib/Text.xtl")),
            ("Svg", include_str!("../../../../lib/Svg.xtl")),
            ("dungeon.toml", include_str!("../../dungeon.toml")),
        ],
        toml: include_str!("../../game.toml"),
    }
}
