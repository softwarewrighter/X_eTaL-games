//! Robot chase in the browser, exactly as X_eTaL runs it: the shared
//! game page (shared/microscope) with this game's files, the same ones
//! the command line runs.

use microscope::page::Game;

pub fn game() -> Game {
    Game {
        title: "Robot chase",
        lede: "The COR24 BASIC robot chase in X_eTaL: twelve robots step toward you every turn, all at once, as one 2 by 12 matrix plus the sign of the difference. Make them crash into each other or into wrecks. Keypad moves (7 8 9 / 4 5 6 / 1 2 3), 0 teleports, 10 scans, 99 resigns.",
        play: include_str!("../../play.xtl"),
        script_name: "robot-chase.xtl",
        script: include_str!("../../robot-chase.xtl"),
        library_name: "RobotChase",
        library: include_str!("../../RobotChase.xtl"),
        shared: &[("Board", include_str!("../../../../lib/Board.xtl")), ("Play", include_str!("../../../../lib/Play.xtl"))],
        toml: include_str!("../../game.toml"),
    }
}
