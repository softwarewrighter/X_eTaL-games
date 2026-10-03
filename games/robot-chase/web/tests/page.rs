//! The page shows what the command line shows: its terminal, given the
//! moves in expected/play.in on seed 1, prints expected/play.out; its
//! notebook prints expected/SLUG.out.

use microscope::terminal::{notebook, session, Line};
use robot_chase_web::game;

fn printed(lines: &[Line]) -> String {
    lines.iter().filter_map(|l| match l {
        Line::Out(s) => Some(format!("{s}\n")),
        _ => None,
    }).collect()
}

#[test]
fn the_terminal_matches_the_command_line_golden() {
    let g = game();
    let typed: Vec<String> = include_str!("../../expected/play.in").lines().map(str::to_string).collect();
    let t = session(&[(g.library_name, g.library)], g.play, &typed, 1);
    assert!(t.error.is_none(), "{:?}", t.error);
    assert!(!t.waiting, "the session in play.in ends the game");
    assert_eq!(printed(&t.lines), include_str!("../../expected/play.out"));
    let read: Vec<String> = t.lines.iter().filter_map(|l| match l { Line::In(s) => Some(s.clone()), _ => None }).collect();
    assert_eq!(read, typed, "every typed line was read, in order");
}

#[test]
fn the_notebook_matches_the_command_line_golden() {
    let g = game();
    let nb = notebook(&[(g.library_name, g.library)], g.script, 1);
    assert!(nb.error.is_none(), "{:?}", nb.error);
    let out: String = nb.cells.iter().map(|c| printed(&c.output)).collect();
    assert_eq!(out, include_str!("../../expected/robot-chase.out"));
}

#[test]
fn a_new_game_waits_for_its_first_line() {
    let g = game();
    let t = session(&[(g.library_name, g.library)], g.play, &[], 7);
    assert!(t.waiting && t.error.is_none());
}

#[test]
fn the_board_is_also_a_picture_x_etal_draws() {
    let g = game();
    let typed: Vec<String> = include_str!("../../expected/play.in").lines().map(str::to_string).collect();
    let t = session(&[(g.library_name, g.library)], g.play, &typed, 1);
    let pictures: Vec<&String> = t.lines.iter().filter_map(|l| match l { Line::Picture(svg) => Some(svg), _ => None }).collect();
    assert!(!pictures.is_empty(), "play.xtl shows the board with []S_HOW");
    assert!(pictures.iter().all(|svg| svg.starts_with("<svg") && svg.contains("<text")), "a grid of characters");
}
