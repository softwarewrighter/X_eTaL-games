//! Shut the box in the browser, exactly as X_eTaL runs it: the shared
//! game page (shared/microscope) with this game's files, the same ones
//! the command line runs.

use microscope::page::Game;

pub fn game() -> Game {
    Game {
        title: "Shut the box",
        lede: "Roll the dice and shut tiles adding up to the roll; shut all nine to win. In X_eTaL every set of tiles is considered at once: the 512 subsets of 1 to 9 are the columns of a 9 by 512 matrix of bits, their sums one inner product, and the legal moves the columns that fit and add up to the roll.",
        play: include_str!("../../play.xtl"),
        interactive: false,
        ticks: false,
        script_name: "shut-the-box.xtl",
        script: include_str!("../../shut-the-box.xtl"),
        library_name: "ShutTheBox",
        library: include_str!("../../ShutTheBox.xtl"),
        shared: &[("Board", include_str!("../../../../lib/Board.xtl")), ("Play", include_str!("../../../../lib/Play.xtl")), ("Text", include_str!("../../../../lib/Text.xtl"))],
        toml: include_str!("../../game.toml"),
    }
}
