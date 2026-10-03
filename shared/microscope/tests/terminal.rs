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


#[test]
fn a_notebook_gives_each_statement_then_its_output() {
    let src = "# two numbers\nx := 1 2\nx + 1\n# and a sum\n'+ r_/ x\n";
    let nb = microscope::terminal::notebook(&[], src, 1);
    assert!(nb.error.is_none());
    let outs: Vec<Vec<Line>> = nb.cells.iter().map(|c| c.output.clone()).collect();
    assert_eq!(nb.cells.len(), 3, "{:?}", nb.cells);
    assert!(nb.cells[1].source.contains("x + 1"));
    assert_eq!(outs[1], vec![Line::Out("2 3".into())]);
    assert!(nb.cells[2].source.starts_with("# and a sum"));
    assert_eq!(outs[2], vec![Line::Out("3".into())]);
}

#[test]
fn pictures_come_from_the_program() {
    let t = session(&[], "x := []S_HOW []G_RID 1 0\n", &[], 1);
    assert!(matches!(t.lines.first(), Some(Line::Picture(svg)) if svg.starts_with("<svg")));
}
