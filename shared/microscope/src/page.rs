//! A game's page, showing only what X_eTaL prints or draws: the game's
//! terminal program (`play.xtl`) running in a terminal, its scripted
//! program as a notebook (each statement, then its output, as `just
//! show`), and the sources, decorated by X_eTaL's own renderer. The
//! page draws nothing computed from the game: no boards, highlights or
//! verdicts of its own. Its only controls start a new game or restart
//! this one.

use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlDialogElement, HtmlInputElement};
use yew::prelude::*;

use crate::chrome::footer;
use crate::source::{block, NONE};
use crate::terminal::{notebook, session, Line};

/// What a game page shows.
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Game {
    pub title: &'static str,
    pub lede: &'static str,
    /// The terminal program, `play.xtl`.
    pub play: &'static str,
    /// Whether `play.xtl` is played by clicking its pictures (a map, a
    /// board) rather than typing: the page shows its latest picture in a
    /// large dialog and gives it each click as an event line,
    /// `click X Y` in the picture's own coordinates ([]E_VENT).
    pub interactive: bool,
    /// The scripted program's file name and text (`<slug>.xtl`).
    pub script_name: &'static str,
    pub script: &'static str,
    /// The rules library's name (as `u_se<` names it) and text.
    pub library_name: &'static str,
    pub library: &'static str,
    /// The shared libraries (lib/) the game uses, by name and text, and
    /// its data files, by path (with the extension) and text.
    pub shared: &'static [(&'static str, &'static str)],
    /// The game's game.toml: its `wikipedia` link or its `about` text
    /// (what the title opens).
    pub toml: &'static str,
}

/// A string field of a game.toml (`key = "value"` on one line, with \"
/// and \\ escapes), if it is there.
pub fn field(toml: &str, key: &str) -> Option<String> {
    toml.lines().find_map(|line| {
        let rest = line.trim_start().strip_prefix(key)?.trim_start().strip_prefix('=')?.trim_start();
        let mut chars = rest.strip_prefix('"')?.chars();
        let mut out = String::new();
        while let Some(c) = chars.next() {
            match c {
                '\\' => out.push(chars.next()?),
                '"' => return Some(out),
                c => out.push(c),
            }
        }
        None
    })
}

/// A small page glyph (a sheet with a folded corner) marking a link to
/// an article elsewhere.
fn page_glyph() -> Html {
    html! {
        <svg class="glyph" viewBox="0 0 16 16" width="16" height="16" aria-hidden="true">
            <path d="M3 1.5h6.5L13 5v9.5H3z" fill="none" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round"/>
            <path d="M9.5 1.5V5H13M5.5 8h5M5.5 10.5h5M5.5 13h3" fill="none" stroke="currentColor" stroke-width="1.1"/>
        </svg>
    }
}

#[derive(Properties, PartialEq)]
struct TitleProps {
    title: &'static str,
    wikipedia: Option<String>,
    about: Option<String>,
}

/// The game's title: a link to its Wikipedia article (new tab, page
/// glyph) when it has one, else a button opening a dialog on its
/// history and play, closed by Escape, a click outside it, or its X.
#[function_component(Title)]
fn title(p: &TitleProps) -> Html {
    let dialog = use_node_ref();
    if let Some(url) = &p.wikipedia {
        return html! {
            <h1><a class="wiki" href={url.clone()} target="_blank" rel="noopener noreferrer" title="Wikipedia (opens in a new tab)">{p.title}{" "}{page_glyph()}</a></h1>
        };
    }
    let open = {
        let dialog = dialog.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(d) = dialog.cast::<HtmlDialogElement>() {
                let _ = d.show_modal();
            }
        })
    };
    let close = {
        let dialog = dialog.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(d) = dialog.cast::<HtmlDialogElement>() {
                d.close();
            }
        })
    };
    let outside = {
        let dialog = dialog.clone();
        Callback::from(move |e: MouseEvent| {
            let on_backdrop = e.target().and_then(|t| t.dyn_into::<Element>().ok()).is_some_and(|t| t.tag_name() == "DIALOG");
            if let Some(d) = dialog.cast::<HtmlDialogElement>().filter(|_| on_backdrop) {
                d.close();
            }
        })
    };
    html! { <>
        <h1><button class="about-open" onclick={open} title="About this game">{p.title}</button></h1>
        <dialog class="about" ref={dialog} onclick={outside}>
            <div class="about-box">
                <button class="close" aria-label="Close" onclick={close}>{"\u{00d7}"}</button>
                <h2>{p.title}</h2>
                <p>{p.about.clone().unwrap_or_default()}</p>
            </div>
        </dialog>
    </> }
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

#[derive(Properties, PartialEq)]
struct BoardProps {
    game: Game,
    seed: u64,
    typed: Vec<String>,
    on_line: Callback<String>,
}

/// Where a click on a picture falls in the picture's own coordinates:
/// its SVG's viewBox, drawn at full width in its own proportions.
fn picture_point(e: &MouseEvent) -> Option<(f64, f64)> {
    let target = e.target()?.dyn_into::<Element>().ok()?;
    let svg = target.closest("svg").ok()??;
    let vb: Vec<f64> = svg.get_attribute("viewBox")?.split_whitespace().filter_map(|n| n.parse().ok()).collect();
    let r = svg.get_bounding_client_rect();
    (vb.len() == 4 && r.width() > 0.0 && r.height() > 0.0).then(|| {
        (vb[0] + (e.client_x() as f64 - r.left()) / r.width() * vb[2], vb[1] + (e.client_y() as f64 - r.top()) / r.height() * vb[3])
    })
}

/// An interactive game: a button opening a large dialog (closed by
/// Escape, its X, or a click on the background) with the program's
/// latest picture; a click on it is given to the program as an event
/// line and the program is run again with every click so far.
#[function_component(Board)]
fn board(p: &BoardProps) -> Html {
    let t = use_memo((p.seed, p.typed.clone()), |(seed, typed)| session(&p.game.libraries(), p.game.play, typed, *seed));
    let dialog = use_node_ref();
    {
        // An address ending in #play opens the game at once (a link to
        // play it; the screenshots use it).
        let dialog = dialog.clone();
        use_effect_with((), move |_| {
            #[cfg(target_arch = "wasm32")]
            if web_sys::window().and_then(|w| w.location().hash().ok()).is_some_and(|h| h == "#play") {
                if let Some(d) = dialog.cast::<HtmlDialogElement>() {
                    let _ = d.show_modal();
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            let _ = &dialog;
        });
    }
    let open = {
        let dialog = dialog.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(d) = dialog.cast::<HtmlDialogElement>() {
                let _ = d.show_modal();
            }
        })
    };
    let close = {
        let dialog = dialog.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(d) = dialog.cast::<HtmlDialogElement>() {
                d.close();
            }
        })
    };
    let onclick = {
        let (dialog, on_line) = (dialog.clone(), p.on_line.clone());
        Callback::from(move |e: MouseEvent| {
            let on_backdrop = e.target().and_then(|t| t.dyn_into::<Element>().ok()).is_some_and(|t| t.tag_name() == "DIALOG");
            if on_backdrop {
                if let Some(d) = dialog.cast::<HtmlDialogElement>() {
                    d.close();
                }
            } else if let Some((x, y)) = picture_point(&e) {
                on_line.emit(format!("click {} {}", x.round(), y.round()));
            }
        })
    };
    let picture = t.lines.iter().rev().find_map(|l| match l { Line::Picture(svg) => Some(svg.clone()), _ => None });
    let said: Vec<&String> = t.lines.iter().filter_map(|l| match l { Line::Out(s) => Some(s), _ => None }).collect();
    html! {
        <>
        <button class="board-open" onclick={open}>{"Play"}</button>
        <dialog class="board" ref={dialog} {onclick}>
            <button class="close" aria-label="Close" onclick={close}>{"\u{00d7}"}</button>
            // An error stops the program: said first, above the picture it
            // left, so a game that stopped never looks merely unresponsive.
            if let Some(e) = &t.error { <div class="err board-err">{format!("The program stopped: {e}")}</div> }
            if let Some(svg) = picture { <div class="board-pic">{ Html::from_html_unchecked(AttrValue::from(svg)) }</div> }
            <pre class="transcript" hidden=true>{ for said.iter().map(|s| format!("{s}\n")) }</pre>
        </dialog>
        </>
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
            <div class="brand">
                <a href="../" title="All games"><img class="logo" src="modern-xetal-logo.jpg" alt="X_eTaL" /></a>
                <Title title={g.title} wikipedia={field(g.toml, "wikipedia")} about={field(g.toml, "about")} />
            </div>
            <p class="lede">{g.lede}</p>
            // A game played by clicks starts its own rounds on the map, so its
            // page has only Play (in the panel below), no New game or
            // Restart (the user, 2026-10-07).
            if !g.interactive {
                <div class="controls">
                    <button onclick={new_game}>{"New game"}</button>
                    <button onclick={restart}>{"Restart this game"}</button>
                    <span class="gen">{format!("seed {}", *seed)}</span>
                </div>
            }
        </header>
        <main>
            <div class="layout even">
                <div class="col">
                    if g.interactive {
                        <section class="panel">
                            <h2>{"Play: play.xtl"}</h2>
                            <p class="note">{"The game's program, unmodified, run by X_eTaL in your browser. Play opens its picture, drawn by X_eTaL; each click is given to the program as an event, click X Y in the picture's coordinates, and the program is run again from the start with every click so far, on the same seed, so it is the same game. Close it with Escape, the X or a click outside; Play again to go on."}</p>
                            <Board game={g.clone()} seed={*seed} typed={(*typed).clone()} {on_line} />
                        </section>
                    } else {
                        <section class="panel">
                            <h2>{"Play: play.xtl"}</h2>
                            <p class="note">{"The game's terminal program, unmodified, run by X_eTaL in your browser. Type a line and press Enter: the program is run again from the start with everything typed so far as its keyboard, on the same seed, so it is the same game."}</p>
                            <Terminal game={g.clone()} seed={*seed} typed={(*typed).clone()} {on_line} />
                        </section>
                    }
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
                    { for g.shared.iter().map(|(name, text)| if name.contains("assets/cache/") { html! {
                        <section class="panel code">
                            <h2>{format!("Data: {name}")}</h2>
                            <p class="note">{format!("Fetched from its source and converted by the game's assets/fetch.sh when the site is built (never tracked: third-party data); {} KB, not shown.", text.len() / 1024)}</p>
                        </section>
                    } } else if name.contains('.') { html! {
                        <section class="panel code">
                            <h2>{format!("Data: {name}")}</h2>
                            <p class="note">{"A data file the program reads (TOML, with []L_IST and []T_ABLE)."}</p>
                            <pre class="data">{text}</pre>
                        </section>
                    } } else { html! {
                        <section class="panel code">
                            <h2>{format!("A shared library: {name}.xtl")}</h2>
                            <p class="note">{"Used by several games; the games' programs import it with u_se<."}</p>
                            { block(text, NONE) }
                        </section>
                    } }) }
                </div>
            </div>
        </main>
        { footer() }
        </>
    }
}
