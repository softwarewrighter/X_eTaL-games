use microscope::terminal::{session, Line};

const ECHO: &str = "\"e:\" u_se< \"Echo\"\nu:l_oop := { n ->\n  shown := p_rint! \"say something:\"\n  t := []R_EAD @\n  t m_atch \"bye\" ? n; u:l_oop n + e:one\n}\n\"lines: \" c_at f_ormat u:l_oop 0\n";
const LIB: (&str, &str) = ("Echo", "l:one := 1\n");

fn typed(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

#[test]
fn a_program_waits_for_the_next_line() {
    let t = session(&[LIB], ECHO, &typed(&["hi"]), 1);
    assert!(t.waiting && t.error.is_none());
    assert_eq!(t.lines, vec![
        Line::Out("say something:".into()),
        Line::In("hi".into()),
        Line::Out("say something:".into()),
    ]);
}

#[test]
fn a_finished_program_is_neither_waiting_nor_failed() {
    let t = session(&[LIB], ECHO, &typed(&["a", "b", "bye"]), 1);
    assert!(!t.waiting && t.error.is_none());
    assert_eq!(t.lines.last(), Some(&Line::Out("lines: 2".into())));
}

#[test]
fn an_error_is_reported() {
    let t = session(&[], ECHO, &[], 1);
    assert!(t.error.is_some(), "the library is missing");
}

#[test]
fn the_same_seed_replays_the_same_game() {
    let src = "x := r_oll! 1000000\nshown := p_rint! x\nt := []R_EAD @\nx\n";
    let a = session(&[], src, &typed(&["go"]), 9);
    assert_eq!(a, session(&[], src, &typed(&["go"]), 9));
    assert_ne!(a, session(&[], src, &typed(&["go"]), 10));
}

