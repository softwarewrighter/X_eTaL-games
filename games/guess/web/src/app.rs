//! The page: the terminal, the hundred candidates, the rules.

use web_sys::{Element, HtmlInputElement};
use yew::prelude::*;

use microscope::chrome::{chip, footer, header, notice, panel};
use microscope::source::code;
use microscope::terminal::Line;

use crate::micro::{guesses, TOP};
use crate::model::{Action, Model};
use crate::view::{program, source, Stage, STAGES};

fn act(m: &UseReducerHandle<Model>, a: impl Fn() -> Action + 'static) -> Callback<MouseEvent> {
    let d = m.dispatcher();
    Callback::from(move |_| d.dispatch(a()))
}

fn stage_chip(m: &UseReducerHandle<Model>, s: Stage) -> Html {
    let (name, src, dims, meaning): (&str, &str, Vec<usize>, &str) = match s {
        Stage::Secret => ("secret", "g:n_ew g:top", vec![], "one number, 1 to 100"),
        Stage::Answer => ("answer", "t g:m_ove g", vec![], "-1 higher, 1 lower, 0 right (for every guess at once)"),
        Stage::Reply => ("reply", "g:r_eply r", vec![], "the answer as text, as the BASIC printed it"),
        Stage::Possible => ("possible", "t g:p_ossible gs", vec![TOP], "a 0/1 mask of the secrets still possible"),
    };
    chip(name, src, &dims, meaning, m.focus == s, act(m, move || Action::Focus(s)))
}

#[function_component(Terminal)]
fn terminal() -> Html {
    let model = use_context::<UseReducerHandle<Model>>().expect("the model");
    let input = use_node_ref();
    let pane = use_node_ref();
    let onsubmit = {
        let d = model.dispatcher();
        let input = input.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if let Some(el) = input.cast::<HtmlInputElement>() {
                d.dispatch(Action::Type(el.value()));
                el.set_value("");
            }
        })
    };
    {
        // Keep the newest lines in view inside the pane (not the page),
        // and the cursor in the input once you have typed.
        let (input, pane) = (input.clone(), pane.clone());
        use_effect_with(model.typed.len(), move |&n| {
            if let Some(div) = pane.cast::<Element>() {
                div.set_scroll_top(div.scroll_height());
            }
            if let Some(el) = input.cast::<HtmlInputElement>().filter(|_| n > 0) {
                let _ = el.focus();
            }
        });
    }
    let lines = model.transcript.lines.iter().map(|l| match l {
        Line::Out(s) => html! { <div>{s}</div> },
        Line::In(s) => html! { <div class="in">{format!("> {s}")}</div> },
    });
    html! {
        <div class="term" ref={pane}>
            { for lines }
            if model.transcript.waiting {
                <form {onsubmit}><span class="in">{">"}</span><input ref={input} type="text" inputmode="numeric" autocomplete="off" aria-label="your guess" /></form>
            } else {
                <div class="done">{"-- game over: New game to play again --"}</div>
            }
        </div>
    }
}

fn candidates(m: &Model) -> Html {
    let gs = guesses(&m.transcript);
    let cells = (1..=TOP as i64).map(|x| {
        let i = gs.iter().rposition(|&g| g == x);
        let class = match i.map(|i| m.board.answers.get(i).copied().unwrap_or(9)) {
            Some(0) => "hit",
            Some(-1) => "lo",
            Some(1) => "hi",
            _ if m.board.possible[x as usize - 1] => "ok",
            _ => "",
        };
        html! { <span {class}>{x}</span> }
    });
    let left = m.board.possible.iter().filter(|&&b| b).count();
    let answers: Vec<String> = m.board.answers.iter().map(i64::to_string).collect();
    let gtext: Vec<String> = gs.iter().map(i64::to_string).collect();
    let body = html! { <>
        <div class="line100">{ for cells }</div>
        if !gs.is_empty() {
            <p class="calc">{"Your guesses: "}{code(&gtext.join(" "))}{"; their answers, all at once: "}{code(&answers.join(" "))}</p>
        }
        <p class="note">{format!("{left} secret(s) still possible (green). Blue: guessed too low; orange: too high. ")}{"The mask comes from a table of every candidate against every guess, rows kept where every answer agrees: no loop."}</p>
    </> };
    panel("Still possible:", "t g:p_ossible gs", "", matches!(m.focus, Stage::Possible | Stage::Answer), body)
}

#[function_component(App)]
pub fn app() -> Html {
    let model = use_reducer(Model::new);
    let m: &Model = &model;
    html! {
        <ContextProvider<UseReducerHandle<Model>> context={model.clone()}>
        <header>
            { header("Guess the number", "The smallest game of the COR24 BASIC demos, in X_eTaL: I am thinking of a number from 1 to 100; you guess, I say higher or lower. The terminal below runs the same play.xtl as the command line.") }
            <nav class="timeline">{ for STAGES.iter().map(|&s| stage_chip(&model, s)) }</nav>
            <div class="controls">
                <button onclick={act(&model, || Action::NewGame)}>{"New game"}</button>
                <span class="gen">{format!("{} guess(es)", guesses(&m.transcript).len())}</span>
            </div>
            { notice(&m.notice) }
        </header>
        <main>
            <div class="layout even">
                <div class="col">
                    { panel("The terminal:", "play.xtl", "Type a number and press Enter.", false, html! { <Terminal /> }) }
                    { candidates(m) }
                </div>
                <div class="col">
                    <section class="panel code">
                        <h2>{"The program: play.xtl"}</h2>
                        <p class="note">{"The terminal game, the same file the command line plays, run by X_eTaL in your browser with what you typed as its keyboard. It imports the rules library as g:. The stage you pick is highlighted."}</p>
                        { program(m.focus) }
                    </section>
                    <section class="panel code">
                        <h2>{"The library: NumberGuess.xtl"}</h2>
                        <p class="note">{"The rules, written once. A library names its exports l: (\"this library\"); a program that imports it with \"g:\" u_se< \"NumberGuess\" calls them as g:."}</p>
                        { source(m.focus) }
                    </section>
                </div>
            </div>
        </main>
        { footer() }
        </ContextProvider<UseReducerHandle<Model>>>
    }
}
