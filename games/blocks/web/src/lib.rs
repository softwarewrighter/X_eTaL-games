//! Falling blocks in the browser, exactly as X_eTaL runs it: the shared
//! game page (shared/microscope) with this game's files and data, the
//! same ones the command line runs.

use microscope::page::Game;

pub fn game() -> Game {
    Game {
        title: "Falling blocks",
        lede: "Fill whole rows to clear them: left and right move the piece, up turns it, down moves it a row, the space bar drops it, h lets the computer player place it. The game moves with the clock: the page gives the program a tick twenty times a second and your keys, and shows X_eTaL's picture. The well is a matrix and a piece a small square of 0s and 1s, turned by reversing its rows and transposing.",
        play: include_str!("../../play.xtl"),
        interactive: true,
        ticks: true,
        script_name: "blocks.xtl",
        script: include_str!("../../blocks.xtl"),
        library_name: "Stack",
        library: include_str!("../../Stack.xtl"),
        shared: &[
            ("Text", include_str!("../../../../lib/Text.xtl")),
            ("Svg", include_str!("../../../../lib/Svg.xtl")),
            ("blocks.toml", include_str!("../../blocks.toml")),
        ],
        toml: include_str!("../../game.toml"),
    }
}
