//! The page shows what the command line shows: its terminal, given the
//! moves in expected/play.in on seed 1, prints expected/play.out; its
//! notebook prints expected/SLUG.out.

use microscope::terminal::{notebook, session, Line};
use dungeon_web::game;

fn printed(lines: &[Line]) -> String {
    lines.iter().filter_map(|l| match l {
        Line::Out(s) => Some(format!("{s}\n")),
        _ => None,
    }).collect()
}

#[test]
fn the_clicks_replay_the_command_line_golden() {
    let g = game();
    let typed: Vec<String> = include_str!("../../expected/play.in").lines().map(str::to_string).collect();
    let t = session(&g.libraries(), g.play, &typed, 1);
    assert!(t.error.is_none(), "{:?}", t.error);
    assert!(!t.waiting, "the session in play.in ends the game");
    assert_eq!(printed(&t.lines), include_str!("../../expected/play.out"));
    let read: Vec<String> = t.lines.iter().filter_map(|l| match l { Line::In(s) => Some(s.clone()), _ => None }).collect();
    assert_eq!(read, typed, "every typed line was read, in order");
}

#[test]
fn the_notebook_matches_the_command_line_golden() {
    let g = game();
    let nb = notebook(&g.libraries(), g.script, 1);
    assert!(nb.error.is_none(), "{:?}", nb.error);
    let out: String = nb.cells.iter().map(|c| printed(&c.output)).collect();
    assert_eq!(out, include_str!("../../expected/dungeon.out"));
}

#[test]
fn a_new_game_shows_its_map() {
    // Played by clicks (Game::interactive): with no click yet the program
    // draws its first map, then reads the end of the events.
    let g = game();
    assert!(g.interactive);
    let t = session(&g.libraries(), g.play, &[], 7);
    assert!(t.error.is_none(), "{:?}", t.error);
    let svg = t.lines.iter().find_map(|l| match l { Line::Picture(s) => Some(s.clone()), _ => None }).expect("a picture");
    assert!(svg.contains("viewBox=") && svg.contains("class=\"entity\""));
}

#[test]
fn clicks_fed_one_at_a_time_give_the_replayed_game() {
    // The page keeps the program running between clicks (Live); each
    // click fed to it must leave the same transcript as a replay of every
    // click from the start, but for the end of events the replay reads.
    use microscope::terminal::Live;
    let g = game();
    let clicks: Vec<String> = include_str!("../../expected/play.in").lines().map(str::to_string).collect();
    let mut live = Live::start(&g.libraries(), g.play, 1);
    for c in &clicks {
        live.feed(c);
    }
    let t = live.transcript();
    assert!(t.error.is_none() && t.waiting, "{:?}", t.error);
    let replay = session(&g.libraries(), g.play, &clicks, 1);
    let mut want = replay.lines.clone();
    assert_eq!(want.pop(), Some(Line::Out("THE END".into())));
    assert_eq!(t.lines, want);
}
