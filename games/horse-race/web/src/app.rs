//! The page: the track X_eTaL draws, this round's rolls, the rules.

use gloo_timers::callback::Interval;
use yew::prelude::*;

use microscope::chrome::{chip, footer, header, notice, panel};
use microscope::source::code;

use crate::micro::HORSES;
use crate::model::{Action, Model};
use crate::view::{source, Stage, STAGES};

const NAMES: [&str; HORSES] = ["Lucky", "Thunder", "Shadow", "Comet", "Blaze"];

fn act(m: &UseReducerHandle<Model>, a: impl Fn() -> Action + 'static) -> Callback<MouseEvent> {
    let d = m.dispatcher();
    Callback::from(move |_| d.dispatch(a()))
}

fn stage_chip(m: &UseReducerHandle<Model>, s: Stage) -> Html {
    let (name, src, dims, meaning): (&str, &str, Vec<usize>, &str) = match s {
        Stage::Roll => ("roll", "r_oll! (t_ally p) r_eshape 3", vec![HORSES], "one roll per horse"),
        Stage::Move => ("move", "p + r", vec![HORSES], "every position, moved at once"),
        Stage::Finish => ("finish", "'| r_/ p >= l:finish", vec![], "one number: is any horse home?"),
        Stage::Winners => ("winners", "w_here p = 'm_ax r_/ p", vec![], "the leaders' numbers (one or more)"),
        Stage::Track => ("track", "l:v_iew p", vec![HORSES, 28], "a character matrix: names, rail, bars"),
    };
    chip(name, src, &dims, meaning, m.focus == s, act(m, move || Action::Focus(s)))
}

fn controls(m: &UseReducerHandle<Model>) -> Html {
    let go = if m.playing { "Racing..." } else if m.over() || m.rounds > 0 { "Race again" } else { "Start the race" };
    html! {
        <div class="controls">
            <button onclick={act(m, || Action::Go)} disabled={m.playing}>{go}</button>
            <button onclick={act(m, || Action::Step)}>{"One round"}</button>
            <button onclick={act(m, || Action::NewRace)}>{"Reset"}</button>
            <span class="gen">{format!("round {}", m.rounds)}</span>
        </div>
    }
}

fn track(m: &UseReducerHandle<Model>) -> Html {
    let winners = m.last.as_ref().map(|r| r.winners.clone()).unwrap_or_default();
    html! {
        <pre class="track">
            { for m.track.iter().enumerate().map(|(i, row)| {
                let class = classes!("row", (m.pick == Some(i)).then_some("pick"), winners.get(i).copied().unwrap_or(false).then_some("win"));
                html! { <span {class} onclick={act(m, move || Action::Pick(i))}>{row}</span> }
            }) }
        </pre>
    }
}

fn result(m: &Model) -> Html {
    let Some(r) = m.last.as_ref().filter(|r| r.over) else {
        let ask = match m.pick {
            Some(i) => format!("You are on {}. Start the race.", NAMES[i]),
            None => "Click a horse to back it, then start the race.".to_string(),
        };
        return html! { <p class="note">{ask}</p> };
    };
    let won: Vec<&str> = (0..HORSES).filter(|&i| r.winners[i]).map(|i| NAMES[i]).collect();
    let verdict = match m.pick {
        Some(i) if r.winners[i] => format!(" Your horse, {}, won!", NAMES[i]),
        Some(i) => format!(" Your horse, {}, lost.", NAMES[i]),
        None => String::new(),
    };
    html! { <p class="result">{format!("Winner: {}.{verdict}", won.join(" and "))}</p> }
}

fn round_panel(m: &Model) -> Html {
    let body = match &m.last {
        None => html! { <p class="note">{"No round yet: every horse is at 0."}</p> },
        Some(r) => html! { <>
            <p class="calc">{"The rolls, one per horse, all at once: "}{code("r := h:r_oll p")}</p>
            <div class="rolls">{ for r.rolls.iter().map(|x| html! { <span>{x}</span> }) }</div>
            <p class="calc">{"The new positions: "}{code("p h:m_ove r")}{" = "}{code(&r.pos.iter().map(i64::to_string).collect::<Vec<_>>().join(" "))}</p>
            <p class="calc">{"Is any horse home? "}{code("'| r_/ p >= l:finish")}{" = "}{code(if r.over { "1" } else { "0" })}</p>
            <p class="note">{"The whole field is one vector: a round is one addition, whether there are five horses or five thousand."}</p>
        </> },
    };
    panel("This round:", "p + r_oll! 5 r_eshape 3", "", matches!(m.focus, Stage::Roll | Stage::Move | Stage::Finish), body)
}

#[function_component(App)]
pub fn app() -> Html {
    let model = use_reducer(Model::new);
    {
        let d = model.dispatcher();
        use_effect_with(model.playing, move |&playing| {
            let timer = playing.then(|| Interval::new(450, move || d.dispatch(Action::Tick)));
            move || drop(timer)
        });
    }
    let m: &Model = &model;
    html! {
        <>
        <header>
            { header("Horse race", "Five horses, a track of 15. Every round each horse runs 1, 2 or 3, all at once: the field is one vector of positions, and a round is one X_eTaL expression. Ported from a 1970s-style APL program.") }
            <nav class="timeline">{ for STAGES.iter().map(|&s| stage_chip(&model, s)) }</nav>
            { controls(&model) }
            { notice(&m.notice) }
        </header>
        <main>
            <div class="layout even">
                <div class="col">
                    { panel("The track:", "h:v_iew p", "Drawn by X_eTaL as a character matrix; click a row to back that horse.", matches!(m.focus, Stage::Track | Stage::Winners), html! { <>{ track(&model) }{ result(m) }</> }) }
                    { round_panel(m) }
                </div>
                <div class="col">
                    <section class="panel code">
                        <h2>{"The rules"}</h2>
                        <p class="note">{"HorseRace.xtl, the library the command-line games use too, run by X_eTaL in your browser; the stage you pick is highlighted."}</p>
                        { source(m.focus) }
                    </section>
                </div>
            </div>
        </main>
        { footer() }
        </>
    }
}
