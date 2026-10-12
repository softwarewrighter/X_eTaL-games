//! Tower defense in the browser, exactly as X_eTaL runs it: the shared
//! game page (shared/microscope) with this game's files and data, the
//! same ones the command line runs.

use microscope::page::Game;

pub fn game() -> Game {
    Game {
        title: "Tower defense",
        lede: "Stop the creeps on the road: pick a tower on the toolbar, click the grass to build it, click a tower to upgrade it, send the waves when ready, and open the grimoire to see every tower and creep. The game moves with the clock: the page gives the program a tick twenty times a second, your clicks and your keys, and shows X_eTaL's picture of the isometric board. Every tower against every creep is one table, on an entity-component system written in X_eTaL.",
        play: include_str!("../../play.xtl"),
        interactive: true,
        ticks: true,
        script_name: "tower-defense.xtl",
        script: include_str!("../../tower-defense.xtl"),
        library_name: "Defense",
        library: include_str!("../../Defense.xtl"),
        shared: &[
            ("Ecs", include_str!("../../../../lib/Ecs.xtl")),
            ("Ecs.xtlm", include_str!("../../../../lib/Ecs.xtlm")),
            ("Text", include_str!("../../../../lib/Text.xtl")),
            ("Svg", include_str!("../../../../lib/Svg.xtl")),
            ("td.toml", include_str!("../../td.toml")),
        ],
        toml: include_str!("../../game.toml"),
    }
}
