//! The horse race in the browser, exactly as X_eTaL runs it: the shared
//! game page (shared/microscope) with this game's files, the same ones
//! the command line runs.

use microscope::page::Game;

pub fn game() -> Game {
    Game {
        title: "Horse race",
        lede: "Five horses race to 15, each running 1, 2 or 3 a round, all at once: the field is one vector of positions and a round is one X_eTaL expression. Bet coins on a horse in the terminal. Ported from a 1970s-style APL program.",
        play: include_str!("../../play.xtl"),
        script_name: "horse-race.xtl",
        script: include_str!("../../horse-race.xtl"),
        library_name: "HorseRace",
        library: include_str!("../../HorseRace.xtl"),
        shared: &[("Play", include_str!("../../../../lib/Play.xtl"))],
    }
}
