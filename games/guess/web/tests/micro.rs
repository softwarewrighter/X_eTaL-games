use guess_web::micro::{board, guesses, replay, PLAY, RULES, TOP};
use microscope::terminal::Line;

fn typed(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

#[test]
fn the_page_runs_the_command_line_game() {
    assert!(PLAY.contains("u_se< \"NumberGuess\""));
    assert!(RULES.contains("l:m_ove := { t g -> (g > t) - g < t }"));
}

#[test]
fn the_game_greets_and_asks() {
    let t = replay(&[], 1);
    assert!(t.waiting && t.error.is_none());
    assert_eq!(t.lines[0], Line::Out("--- GUESS THE NUMBER ---".into()));
    assert_eq!(t.lines.last(), Some(&Line::Out("YOUR GUESS ?".into())));
}

/// Find the secret the way a player would, through the page's model.
fn solve(seed: u64) -> (Vec<String>, i64) {
    let (mut lo, mut hi, mut typed) = (1, 100, vec![]);
    loop {
        let g = (lo + hi) / 2;
        typed.push(g.to_string());
        let b = board(&guesses(&replay(&typed, seed)), seed).unwrap();
        match *b.answers.last().unwrap() {
            0 => return (typed, g),
            -1 => lo = g + 1,
            _ => hi = g - 1,
        }
    }
}

#[test]
fn halving_finds_the_secret_and_the_game_ends() {
    for seed in [1, 2, 3, 77] {
        let (typed, secret) = solve(seed);
        assert!(typed.len() <= 7);
        let t = replay(&typed, seed);
        assert!(!t.waiting && t.error.is_none(), "the game is over");
        assert!(t.lines.contains(&Line::Out("CORRECT! YOU GUESSED IT.".into())));
        let b = board(&guesses(&t), seed).unwrap();
        let left: Vec<usize> = (0..TOP).filter(|&i| b.possible[i]).collect();
        assert_eq!(left, vec![secret as usize - 1], "only the secret is possible");
    }
}

#[test]
fn a_line_without_a_number_is_asked_again() {
    let t = replay(&typed(&["what?", "50"]), 1);
    assert_eq!(guesses(&t), vec![50]);
    let asks = t.lines.iter().filter(|l| **l == Line::Out("YOUR GUESS ?".into())).count();
    assert_eq!(asks, 3);
}

#[test]
fn possible_secrets_agree_with_every_answer() {
    let gs = vec![50, 25, 75];
    let b = board(&gs, 5).unwrap();
    for x in 1..=TOP as i64 {
        let fits = gs.iter().zip(&b.answers).all(|(&g, &a)| (g > x) as i64 - ((g < x) as i64) == a);
        assert_eq!(b.possible[x as usize - 1], fits, "candidate {x}");
    }
}

#[test]
fn every_stage_is_defined_in_the_library_and_called_by_the_game() {
    use guess_web::view::{call, range, Stage, STAGES};
    for s in STAGES {
        let (a, b) = range(s);
        assert!(a < b && RULES[a..b].starts_with("l:"), "{:?} not defined", s);
        let (c, d) = call(s);
        assert!(s == Stage::Possible || (c < d && PLAY[c..d].contains("g:")), "{:?} not called", s);
    }
}
