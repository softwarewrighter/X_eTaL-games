use horse_race_web::micro::{program, round, start, HORSES, RULES};

#[test]
fn the_page_uses_the_command_line_rules() {
    assert!(RULES.contains("l:r_oll := { p -> r_oll! (t_ally p) r_eshape 3 }"));
    assert!(program(&[0; HORSES]).contains("u_se< \"HorseRace\""));
}

#[test]
fn the_track_starts_empty() {
    let t = start().unwrap();
    assert_eq!(t.len(), HORSES);
    assert!(t[0].starts_with("LUCKY  |"));
    assert!(!t.iter().any(|row| row.contains('#')));
}

#[test]
fn a_round_moves_every_horse_one_to_three() {
    let before = vec![3, 0, 7, 2, 5];
    for seed in 1..20 {
        let r = round(&before, seed).unwrap();
        assert!(r.rolls.iter().all(|&x| (1..=3).contains(&x)), "rolls {:?}", r.rolls);
        let moved: Vec<i64> = before.iter().zip(&r.rolls).map(|(p, x)| p + x).collect();
        assert_eq!(r.pos, moved);
        assert!(!r.over && r.winners.iter().all(|&w| !w));
    }
}

#[test]
fn seeds_give_different_races_and_repeat() {
    let a = round(&[0; HORSES], 11).unwrap();
    assert_eq!(a, round(&[0; HORSES], 11).unwrap());
    assert!((12..40).any(|s| round(&[0; HORSES], s).unwrap().rolls != a.rolls));
}

#[test]
fn the_race_ends_at_the_finish_with_the_leaders() {
    let r = round(&[14, 14, 0, 0, 13], 3).unwrap();
    assert!(r.over, "two horses were one short of 15");
    let best = *r.pos.iter().max().unwrap();
    let want: Vec<bool> = r.pos.iter().map(|&p| p == best).collect();
    assert_eq!(r.winners, want);
}

#[test]
fn the_track_draws_each_position() {
    let r = round(&[0, 4, 8, 12, 2], 5).unwrap();
    for (row, &p) in r.track.iter().zip(&r.pos) {
        assert_eq!(row.matches('#').count() as i64, p.min(20), "{row}");
    }
}

#[test]
fn every_stage_is_called_by_the_page_program_and_defined_in_the_library() {
    use horse_race_web::view::{call, range, STAGES};
    let src = program(&[0; HORSES]);
    for s in STAGES {
        assert!(src.contains(call(s)), "{:?} not called", s);
        let (a, b) = range(s);
        assert!(a < b && RULES[a..b].starts_with("l:"), "{:?} not defined", s);
    }
}
