//! Lights out in the browser, exactly as X_eTaL runs it: the shared game
//! page (shared/microscope) with this game's files, the same ones the
//! command line runs.

use microscope::page::Game;

pub fn game() -> Game {
    Game {
        title: "Lights out",
        lede: "A board you click: a press toggles a light and its four neighbors; turn every light off. Hint rings a square of a solution X_eTaL works out by Gaussian elimination over GF(2). The board, the buttons and the words are SVG written by X_eTaL; the page only shows the picture and passes your clicks to the program.",
        play: include_str!("../../play.xtl"),
        interactive: true,
        script_name: "lights-out.xtl",
        script: include_str!("../../lights-out.xtl"),
        library_name: "Lamps",
        library: include_str!("../../Lamps.xtl"),
        shared: &[("Text", include_str!("../../../../lib/Text.xtl")), ("Svg", include_str!("../../../../lib/Svg.xtl"))],
        toml: include_str!("../../game.toml"),
    }
}
