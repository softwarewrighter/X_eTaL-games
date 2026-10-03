//! A game's page, showing only what X_eTaL prints or draws: the game's
//! terminal program (`play.xtl`) running in a terminal, its scripted
//! program as a notebook (each statement, then its output, as `just
//! show`), and the sources, decorated by X_eTaL's own renderer. The
//! page draws nothing computed from the game: no boards, highlights or
//! verdicts of its own. Its only controls start a new game or restart
//! this one.

use web_sys::{Element, HtmlInputElement};
use yew::prelude::*;

use crate::chrome::{footer, header};
use crate::source::{block, NONE};
use crate::terminal::{notebook, session, Line};

/// What a game page shows.
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Game {
    pub title: &'static str,
    pub lede: &'static str,
    /// The terminal program, `play.xtl`.
    pub play: &'static str,
    /// The scripted program's file name and text (`<slug>.xtl`).
    pub script_name: &'static str,
    pub script: &'static str,
    /// The rules library's name (as `u_se<` names it) and text.
    pub library_name: &'static str,
    pub library: &'static str,
    /// The shared libraries (lib/) the game uses, by name and text.
    pub shared: &'static [(&'static str, &'static str)],
}

impl Game {
    /// Every library the game's programs may import.
    pub fn libraries(&self) -> Vec<(&'static str, &'static str)> {
        let mut all = vec![(self.library_name, self.library)];
        all.extend_from_slice(self.shared);
        all
    }
}

/// A seed for a new game: the clock in the browser, fixed natively.
fn clock() -> u64 {
    #[cfg(target_arch = "wasm32")]
    {
        js_sys::Date::now() as u64
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        1
    }
}

/// The first game's seed: `?seed=N` in the address replays that game
/// (the browser tests use `?seed=1`, the goldens' seed); else the clock.
fn first_seed() -> u64 {
    #[cfg(target_arch = "wasm32")]
    {
        let search = web_sys::window().and_then(|w| w.location().search().ok()).unwrap_or_default();
        let given = search.trim_start_matches('?').split('&').find_map(|kv| kv.strip_prefix("seed=")?.parse().ok());
        given.unwrap_or_else(clock)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        1
    }
}

fn line(l: &Line) -> Html {
    match l {
        Line::Out(s) => html! { <div>{s}</div> },
        Line::In(s) => html! { <div class="in">{format!("> {s}")}</div> },
        Line::Picture(svg) => html! { <div class="pic">{ Html::from_html_unchecked(AttrValue::from(svg.clone())) }</div> },
    }
}

#[derive(Properties, PartialEq)]
struct TermProps {
    game: Game,
    seed: u64,
    typed: Vec<String>,
    on_line: Callback<String>,
}

#[function_component(Terminal)]
fn terminal(p: &TermProps) -> Html {
    let t = use_memo((p.seed, p.typed.clone()), |(seed, typed)| session(&p.game.libraries(), p.game.play, typed, *seed));
    let input = use_node_ref();
    let pane = use_node_ref();
    {
        // The newest lines in view inside the pane (never the page), and
        // the cursor in the input once you have typed.
        let (input, pane) = (input.clone(), pane.clone());
        use_effect_with(p.typed.len(), move |&n| {
            if let Some(div) = pane.cast::<Element>() {
                div.set_scroll_top(div.scroll_height());
            }
            if let Some(el) = input.cast::<HtmlInputElement>().filter(|_| n > 0) {
                let _ = el.focus();
            }
        });
    }
    let onsubmit = {
        let (input, on_line) = (input.clone(), p.on_line.clone());
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if let Some(el) = input.cast::<HtmlInputElement>() {
                on_line.emit(el.value());
                el.set_value("");
            }
        })
    };
    html! {
        <div class="term" ref={pane}>
            { for t.lines.iter().map(line) }
            if t.waiting {
                <form {onsubmit}><span class="in">{">"}</span><input ref={input} type="text" autocomplete="off" aria-label="type a line" /></form>
            } else if let Some(e) = &t.error {
                <div class="err">{e}</div>
            } else {
                <div class="done">{"-- the program has ended: New game to play again --"}</div>
            }
        </div>
    }
}

#[function_component(Notebook)]
fn notebook_view(p: &Game) -> Html {
    let nb = use_memo(p.clone(), |g| notebook(&g.libraries(), g.script, 1));
    html! {
        <div class="nb">
            { for nb.cells.iter().map(|c| html! {
                <div class="cell">
                    { block(&c.source, NONE) }
                    if !c.output.is_empty() { <div class="out">{ for c.output.iter().map(line) }</div> }
                </div>
            }) }
            { for nb.error.iter().map(|e| html! { <div class="err">{e}</div> }) }
        </div>
    }
}

/// The page.
#[function_component(GamePage)]
pub fn game_page(g: &Game) -> Html {
    let seed = use_state(first_seed);
    let typed = use_state(Vec::<String>::new);
    let on_line = {
        let typed = typed.clone();
        Callback::from(move |s: String| {
            let mut v = (*typed).clone();
            v.push(s);
            typed.set(v);
        })
    };
    let new_game = {
        let (seed, typed) = (seed.clone(), typed.clone());
        Callback::from(move |_: MouseEvent| {
            seed.set(clock().max(*seed + 1));
            typed.set(vec![]);
        })
    };
    let restart = {
        let typed = typed.clone();
        Callback::from(move |_: MouseEvent| typed.set(vec![]))
    };
    let alias = g.library_name.chars().next().map(|c| c.to_ascii_lowercase()).unwrap_or('g');
    html! {
        <>
        <header>
            { header(g.title, g.lede) }
            <div class="controls">
                <button onclick={new_game}>{"New game"}</button>
                <button onclick={restart}>{"Restart this game"}</button>
                <span class="gen">{format!("seed {}", *seed)}</span>
            </div>
        </header>
        <main>
            <div class="layout even">
                <div class="col">
                    <section class="panel">
                        <h2>{"Play: play.xtl"}</h2>
                        <p class="note">{"The game's terminal program, unmodified, run by X_eTaL in your browser. Type a line and press Enter: the program is run again from the start with everything typed so far as its keyboard, on the same seed, so it is the same game."}</p>
                        <Terminal game={g.clone()} seed={*seed} typed={(*typed).clone()} {on_line} />
                    </section>
                    <section class="panel">
                        <h2>{format!("The scripted game: {}", g.script_name)}</h2>
                        <p class="note">{format!("Run as a notebook, as just show runs it (seed 1): each statement, then what it printed. Everything below the source is X_eTaL's output.")}</p>
                        <Notebook ..g.clone() />
                    </section>
                </div>
                <div class="col">
                    <section class="panel code">
                        <h2>{"The program: play.xtl"}</h2>
                        { block(g.play, NONE) }
                    </section>
                    <section class="panel code">
                        <h2>{format!("The library: {}.xtl", g.library_name)}</h2>
                        <p class="note">{format!("The rules, written once. A library names its exports l: (\"this library\"); a program that imports it with \"{alias}:\" u_se< \"{}\" calls them as {alias}:.", g.library_name)}</p>
                        { block(g.library, NONE) }
                    </section>
                    { for g.shared.iter().map(|(name, text)| html! {
                        <section class="panel code">
                            <h2>{format!("A shared library: {name}.xtl")}</h2>
                            <p class="note">{"Used by several games; the games' programs import it with u_se<."}</p>
                            { block(text, NONE) }
                        </section>
                    }) }
                </div>
            </div>
        </main>
        { footer() }
        </>
    }
}
