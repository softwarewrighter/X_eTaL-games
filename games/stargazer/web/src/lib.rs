//! Stargazer in the browser, exactly as X_eTaL runs it: the shared game
//! page (shared/microscope) with this game's files and data, the same
//! ones the command line runs.

use microscope::page::Game;

pub fn game() -> Game {
    Game {
        title: "Stargazer",
        lede: "A sky you click: five named stars are ringed, mixed bright, middling and faint; click one, then its name among four. X_eTaL reads the Bright Star Catalog and the IAU's star names from TOML and draws the whole sky, menus and all, as SVG; the page only shows the picture and passes your clicks to the program.",
        play: include_str!("../../play.xtl"),
        interactive: true,
        ticks: false,
        script_name: "stargazer.xtl",
        script: include_str!("../../stargazer.xtl"),
        library_name: "Sky",
        library: include_str!("../../Sky.xtl"),
        shared: &[
            ("Text", include_str!("../../../../lib/Text.xtl")),
            ("Svg", include_str!("../../../../lib/Svg.xtl")),
            ("stars.toml", include_str!("../../stars.toml")),
            ("assets/cache/sky.toml", include_str!("../../assets/cache/sky.toml")),
        ],
        toml: include_str!("../../game.toml"),
    }
}
