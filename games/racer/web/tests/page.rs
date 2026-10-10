//! The page shows what the command line shows: its terminal, given the
//! moves in expected/play.in on seed 1, prints expected/play.out; its
//! notebook prints expected/SLUG.out.

use microscope::terminal::{notebook, session, Line};
use racer_web::game;

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
    assert_eq!(out, include_str!("../../expected/racer.out"));
}

#[test]
fn a_new_race_shows_its_track() {
    // Moved by the clock (Game::ticks): before any tick the program draws
    // the track, then reads the end of the events.
    let g = game();
    assert!(g.interactive && g.ticks);
    let t = session(&g.libraries(), g.play, &[], 7);
    assert!(t.error.is_none(), "{:?}", t.error);
    let svg = t.lines.iter().find_map(|l| match l { Line::Picture(s) => Some(s.clone()), _ => None }).expect("a picture");
    assert!(svg.contains("viewBox=") && svg.contains("class=\"entity\""));
}

#[test]
fn ticks_and_keys_fed_one_at_a_time_give_the_replayed_game() {
    // The page keeps the program running (Live) and feeds it each tick and
    // key as it comes; the golden's ticks and keys win the race, which
    // ends the program. Lean, as the page runs it, only the latest picture
    // and no tick is kept; what is printed is the same.
    use microscope::terminal::Live;
    let g = game();
    let lines: Vec<String> = include_str!("../../expected/play.in").lines().map(str::to_string).collect();
    let replay = session(&g.libraries(), g.play, &lines, 1);
    let mut live = Live::start(&g.libraries(), g.play, 1);
    let mut lean = Live::start(&g.libraries(), g.play, 1).lean();
    for l in &lines {
        live.feed(l);
        lean.feed(l);
    }
    let (t, u) = (live.transcript(), lean.transcript());
    assert!(t.error.is_none() && !t.waiting, "{:?}", t.error);
    assert_eq!(t.lines, replay.lines);
    assert_eq!(t.lines.last(), Some(&Line::Out("You won the race!".into())));
    let pictures = |l: &[Line]| l.iter().filter(|l| matches!(l, Line::Picture(_))).count();
    assert_eq!(pictures(&u.lines), 1);
    assert!(!u.lines.iter().any(|l| matches!(l, Line::In(s) if s.starts_with("tick "))));
    assert_eq!(printed(&u.lines), printed(&t.lines));
    let last = |l: &[Line]| l.iter().rev().find(|l| matches!(l, Line::Picture(_))).cloned();
    assert_eq!(last(&u.lines), last(&t.lines));
}
