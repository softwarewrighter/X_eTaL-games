//! Capitals in the browser, exactly as X_eTaL runs it: the shared game
//! page (shared/microscope) with this game's files and data, the same
//! ones the command line runs.

use microscope::page::Game;

pub fn game() -> Game {
    Game {
        title: "Capitals",
        lede: "A map you click: each red dot is a capital inside its unlabeled country; click it, then its name among four nearby cities. X_eTaL reads the countries, capitals and cities from TOML, finds the nearest cities by computing every distance at once, and draws the whole map, menus and all, as SVG; the page only shows the picture and passes your clicks to the program.",
        play: include_str!("../../play.xtl"),
        interactive: true,
        ticks: false,
        script_name: "capitals.xtl",
        script: include_str!("../../capitals.xtl"),
        library_name: "Atlas",
        library: include_str!("../../Atlas.xtl"),
        shared: &[
            ("Text", include_str!("../../../../lib/Text.xtl")),
            ("Svg", include_str!("../../../../lib/Svg.xtl")),
            ("places.toml", include_str!("../../places.toml")),
            ("assets/cache/world.toml", include_str!("../../assets/cache/world.toml")),
        ],
        toml: include_str!("../../game.toml"),
    }
}
