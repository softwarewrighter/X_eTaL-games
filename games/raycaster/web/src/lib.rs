//! The raycaster in the browser, exactly as X_eTaL runs it: the shared
//! game page (shared/microscope) with this game's files and data, the
//! same ones the command line runs.

use microscope::page::Game;

pub fn game() -> Game {
    Game {
        title: "Raycaster",
        lede: "Find the way out of the walls, seen from inside: up and down walk, left and right turn, a and d step sideways; walk into the teal door. The view is X_eTaL's ray casting: one ray for every column of the picture, every ray stepped through the grid at once until all of them hit a wall, drawn as SVG; the page gives the program the clock's ticks and your keys.",
        play: include_str!("../../play.xtl"),
        interactive: true,
        ticks: true,
        script_name: "raycaster.xtl",
        script: include_str!("../../raycaster.xtl"),
        library_name: "Rays",
        library: include_str!("../../Rays.xtl"),
        shared: &[
            ("Text", include_str!("../../../../lib/Text.xtl")),
            ("Svg", include_str!("../../../../lib/Svg.xtl")),
            ("raycaster.toml", include_str!("../../raycaster.toml")),
        ],
        toml: include_str!("../../game.toml"),
    }
}
