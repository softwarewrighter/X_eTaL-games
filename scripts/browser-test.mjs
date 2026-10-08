#!/usr/bin/env node
// Play every game in a real browser and compare with the command line.
// For each game with a web app and an expected/play.in, open its built
// page (pages/<slug>/?seed=1, served as GitHub Pages serves it) in
// headless Chrome, type the lines of expected/play.in into the
// terminal one by one, and require the terminal to show exactly
// expected/play.out (the lines X_eTaL printed, typed lines aside), and
// the page's notebook to show expected/<slug>.out. The browser is
// driven over the DevTools protocol with Node's built-in WebSocket: no
// packages.
//   scripts/browser-test.mjs [SLUG...]      (run `just pages` first)
//   scripts/browser-test.mjs --url https://softwarewrighter.github.io/X_eTaL-games/ [SLUG...]
//                                            (the deployed site instead of pages/)
import { spawn, execFileSync } from "node:child_process";
import { createServer } from "node:net";
import { readFileSync, existsSync, mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const chrome = process.env.CHROME || "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";
// Free ports for the web server and Chrome's DevTools (fixed ones
// collided with a sibling repository's server).
const freePort = () => new Promise((r) => { const s = createServer(); s.listen(0, "127.0.0.1", () => { const p = s.address().port; s.close(() => r(p)); }); });
const webPort = await freePort(), cdpPort = await freePort();
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

// --url BASE: test a deployed site (no local server) instead of pages/.
const argv = process.argv.slice(2);
const urlAt = argv.indexOf("--url");
const live = urlAt >= 0 ? argv.splice(urlAt, 2)[1].replace(/\/?$/, "/") : null;
const slugs = argv.length ? argv
  : execFileSync(join(root, "scripts/games.py"), ["list"], { encoding: "utf8" }).split("\n").filter(Boolean);
const games = slugs.filter((s) => existsSync(join(root, "games", s, "web/Cargo.toml"))
  && existsSync(join(root, "games", s, "expected/play.in")));

const server = live ? { kill() {} } : spawn(join(root, "scripts/serve-pages.sh"), [String(webPort)], { stdio: "ignore" });
const base = live || `http://127.0.0.1:${webPort}/X_eTaL-games/`;
const profile = mkdtempSync(join(tmpdir(), "xetal-games-chrome-"));
const browser = spawn(chrome, ["--headless=new", "--disable-gpu", `--remote-debugging-port=${cdpPort}`,
  `--user-data-dir=${profile}`, "about:blank"], { stdio: "ignore" });
const done = async (code) => {
  server.kill();
  const exited = new Promise((r) => browser.once("exit", r));
  browser.kill();
  await Promise.race([exited, sleep(3000)]);
  try { rmSync(profile, { recursive: true, force: true, maxRetries: 5, retryDelay: 200 }); } catch { /* a temp dir */ }
  process.exit(code);
};

async function json(path, method = "GET") {
  for (let i = 0; i < 100; i++) {
    try { return await (await fetch(`http://127.0.0.1:${cdpPort}${path}`, { method, signal: AbortSignal.timeout(10000) })).json(); }
    catch { await sleep(100); }
  }
  throw new Error("Chrome did not answer on the DevTools port");
}

// One page: a DevTools session with a promise per command.
async function open(url) {
  // A blank tab first, then an explicit navigation, waiting for the load
  // event: an evaluation sent while a tab is still switching from its
  // first page could be dropped without an answer.
  const target = await json(`/json/new?about:blank`, "PUT");
  const ws = new WebSocket(target.webSocketDebuggerUrl);
  await new Promise((r, e) => { ws.onopen = r; ws.onerror = e; setTimeout(() => e(new Error("DevTools did not connect")), 15000); });
  let id = 0; const waiting = new Map(); const events = [];
  ws.onmessage = (m) => {
    const msg = JSON.parse(m.data);
    if (msg.id) { waiting.get(msg.id)?.(msg); waiting.delete(msg.id); } else events.push(msg.method);
  };
  // Every command answers within a minute or fails (a stuck page must fail the test, not hang it).
  const send = (method, params = {}) => new Promise((r, e) => {
    const n = ++id;
    const timer = setTimeout(() => { waiting.delete(n); e(new Error(`DevTools ${method} gave no answer`)); }, 60000);
    waiting.set(n, (msg) => { clearTimeout(timer); r(msg); });
    ws.send(JSON.stringify({ id: n, method, params }));
  });
  const t0 = Date.now();
  await send("Page.enable");
  await send("Page.navigate", { url });
  for (let i = 0; i < 300 && !events.includes("Page.loadEventFired"); i++) await sleep(100);
  debug(`${url}: load event after ${Date.now() - t0} ms (${events.includes("Page.loadEventFired") ? "seen" : "never seen"})`);
  // An evaluation can be lost while the page is still loading (its
  // context replaced): try again, a few times, before giving up.
  const evaluate = async (expression) => {
    for (let tries = 0; ; tries++) {
      const r = await send("Runtime.evaluate", { expression, awaitPromise: true, returnByValue: true });
      if (r.result?.exceptionDetails) throw new Error(r.result.exceptionDetails.exception?.description || "page error");
      if (!r.error && r.result?.result) return r.result.result.value;
      if (tries >= 5) throw new Error(r.error?.message || "the page gave no result");
      await sleep(300);
    }
  };
  // Wait until the page has loaded and its app has drawn something.
  for (let i = 0; i < 100; i++) {
    const r = await send("Runtime.evaluate", { expression: 'document.readyState === "complete" && !!document.querySelector("header, main")', returnByValue: true });
    if (r.result?.result?.value === true) break;
    await sleep(100);
  }
  debug(`${url}: drawn after ${Date.now() - t0} ms`);
  const key = async (k, code) => {
    for (const type of ["rawKeyDown", "keyUp"]) await send("Input.dispatchKeyEvent", { type, key: k, code: k, windowsVirtualKeyCode: code, nativeVirtualKeyCode: code });
  };
  // Closing a page closes its tab too (open tabs would otherwise pile up,
  // one running game each, for the rest of the run).
  // /json/close answers with text, not JSON: a plain request (json()
  // would retry it a hundred times, about 33 s a page).
  const close = async () => { ws.close(); await fetch(`http://127.0.0.1:${cdpPort}/json/close/${target.id}`, { signal: AbortSignal.timeout(5000) }).catch(() => {}); };
  return { evaluate, key, close };
}

// Runs in the page: type each line, waiting for the program to read it.
const PLAY = (lines) => `(async () => {
  const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
  const until = async (f, what) => { for (let i = 0; i < 300; i++) { const v = f(); if (v) return v; await sleep(20); } throw new Error("timed out waiting for " + what); };
  const term = await until(() => document.querySelector(".term"), "the terminal");
  for (const line of ${JSON.stringify(lines)}) {
    const reads = document.querySelectorAll(".term div.in").length;
    const input = await until(() => document.querySelector(".term form input"), "the input line");
    input.value = line;
    input.closest("form").requestSubmit();
    await until(() => document.querySelectorAll(".term div.in").length > reads, "the line to be read");
    await sleep(30);
  }
  await until(() => document.querySelector(".term .done, .term .err"), "the end of the game");
  const out = [...document.querySelector(".term").children].filter((d) => d.tagName === "DIV" && !d.classList.length).map((d) => d.textContent + "\\n").join("");
  const err = document.querySelector(".term .err")?.textContent || "";
  const nb = [...document.querySelectorAll(".nb .out > div:not([class])")].map((d) => d.textContent + "\\n").join("");
  const pics = document.querySelectorAll(".term .pic svg").length;
  return { out, err, nb, pics };
})()`;

// Runs in the page, for a game played by clicks (Game::interactive):
// open its dialog, then for each line "click X Y" (the picture's own
// coordinates) click the matching point on the screen, waiting for the
// program to answer; X_eTaL's transcript and the picture come back.
const CLICK = (lines) => `(async () => {
  const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
  const until = async (f, what) => { for (let i = 0; i < 500; i++) { const v = f(); if (v) return v; await sleep(20); } throw new Error("timed out waiting for " + what); };
  const said = () => document.querySelector("dialog.board pre.transcript")?.textContent || "";
  (await until(() => document.querySelector("button.board-open"), "the Play button")).click();
  await until(() => document.querySelector("dialog.board[open] .board-pic svg"), "the picture in the dialog");
  for (const line of ${JSON.stringify(lines)}) {
    const [, x, y] = line.split(" ").map(Number);
    const before = said();
    const svg = document.querySelector("dialog.board[open] .board-pic svg");
    const vb = svg.getAttribute("viewBox").split(" ").map(Number);
    const r = svg.getBoundingClientRect();
    const cx = r.left + (x - vb[0]) / vb[2] * r.width, cy = r.top + (y - vb[1]) / vb[3] * r.height;
    const el = document.elementFromPoint(cx, cy) || svg;
    el.dispatchEvent(new MouseEvent("click", { bubbles: true, clientX: cx, clientY: cy }));
    await until(() => said() !== before, "the program to answer " + line);
    await sleep(30);
  }
  const err = document.querySelector("dialog.board .err")?.textContent || "";
  const nb = [...document.querySelectorAll(".nb .out > div:not([class])")].map((d) => d.textContent + "\\n").join("");
  return { out: said(), err, nb, pics: document.querySelectorAll("dialog.board .board-pic svg").length, controls: !!document.querySelector(".controls") };
})()`;
const BOARD_OPEN = `(() => { if (!document.querySelector("dialog.board[open]")) document.querySelector("button.board-open").click(); return !!document.querySelector("dialog.board[open]"); })()`;
const BOARD_IS_OPEN = `!!document.querySelector("dialog.board[open]")`;
const BOARD_BACKDROP = `(() => { document.querySelector("dialog.board[open]").click(); return true; })()`;
const BOARD_X = `(() => { document.querySelector("dialog.board[open] .close").click(); return true; })()`;

// The game's dialog closes by Escape, a click on the background, and its X.
async function boardProblems(page) {
  for (const [how, act] of [["Escape", null], ["the background", BOARD_BACKDROP], ["the X", BOARD_X]]) {
    if (!(await page.evaluate(BOARD_OPEN))) return "the game's dialog did not open";
    if (act) await page.evaluate(act); else await page.key("Escape", 27);
    await sleep(150);
    if (await page.evaluate(BOARD_IS_OPEN)) return `${how} did not close the game's dialog`;
  }
  return null;
}

// Runs in the page: the title's about link or dialog. For a dialog, the
// three ways to close it are tried by the caller (Escape is a real key
// press, sent over the DevTools protocol).
const ABOUT = (sel) => `(() => {
  const t = document.querySelector(${JSON.stringify(sel)});
  if (!t) return { kind: "none" };
  if (t.tagName === "A") return { kind: "wiki", href: t.href, target: t.target, glyph: !!t.querySelector("svg.glyph") };
  const d = t.closest("h1, h2").parentElement.querySelector("dialog.about") || document.getElementById(t.dataset.dialog);
  return { kind: "dialog", text: d ? d.textContent.trim().length : 0 };
})()`;
const OPEN = (sel) => `(() => { const t = document.querySelector(${JSON.stringify(sel)}); t.click();
  const d = document.querySelector("dialog.about[open]"); return !!d; })()`;
const IS_OPEN = `!!document.querySelector("dialog.about[open]")`;
const BACKDROP = `(() => { const d = document.querySelector("dialog.about[open]"); d.click(); return true; })()`;
const XBUTTON = `(() => { document.querySelector("dialog.about[open] .close").click(); return true; })()`;

// The title of a page (sel) against game.toml: a Wikipedia link in a new
// tab with the glyph, or a dialog that Escape, the backdrop and the X
// each close. Returns a problem, or "" when all is well.
const debug = (...m) => { if (process.env.BT_DEBUG) console.error(`debug +${((performance.now()) / 1000).toFixed(1)}s:`, ...m); };
async function aboutProblems(page, sel, wiki) {
  debug("about", sel);
  const a = await page.evaluate(ABOUT(sel));
  if (wiki) {
    if (a.kind !== "wiki" || a.href !== wiki || a.target !== "_blank" || !a.glyph) return `title should link to ${wiki} in a new tab with a glyph (got ${JSON.stringify(a)})`;
    return "";
  }
  if (a.kind !== "dialog" || a.text < 40) return `title should open an about dialog (got ${JSON.stringify(a)})`;
  for (const [how, close] of [["Escape", null], ["a click outside", BACKDROP], ["the X", XBUTTON]]) {
    debug("open for", how);
    if (!(await page.evaluate(OPEN(sel)))) return "the dialog did not open";
    debug("close by", how);
    if (close) await page.evaluate(close); else { await page.key("Escape", 27); await sleep(50); }
    if (await page.evaluate(IS_OPEN)) return `${how} did not close the dialog`;
  }
  return "";
}

let failed = 0;
// BENCH_OUT=FILE: also write each page's scripted session time (ms) as JSON.
const timings = {};
try {
  await json("/json/version");
  // The catalog first: every card's title, as the game pages are checked below.
  const cat = await open(base);
  let catProblems = 0;
  for (const slug of slugs) {
    const toml = readFileSync(join(root, "games", slug, "game.toml"), "utf8");
    const wiki = (toml.match(/^wikipedia = "(.*?)"/m) || [])[1];
    const problem = await aboutProblems(cat, `[id="${slug}"] h2 a.wiki, [id="${slug}"] h2 button.about-open`, wiki);
    if (problem) { console.log(`FAIL: catalog ${slug}: ${problem}`); failed++; catProblems++; }
  }
  if (!catProblems) console.log(`ok: catalog (${slugs.length} titles: Wikipedia links and about dialogs)`);
  await cat.close();
  for (const slug of games) {
    const dir = join(root, "games", slug);
    const typed = readFileSync(join(dir, "expected/play.in"), "utf8").split("\n").filter((l, i, a) => i < a.length - 1 || l);
    const page = await open(`${base}${slug}/?seed=1`);
    try {
      const t0 = performance.now();
      const interactive = readFileSync(join(dir, "web/src/lib.rs"), "utf8").includes("interactive: true");
      const got = await page.evaluate(interactive ? CLICK(typed) : PLAY(typed));
      timings[slug] = Math.round(performance.now() - t0);
      const want = readFileSync(join(dir, "expected/play.out"), "utf8");
      const wantNb = readFileSync(join(dir, `expected/${slug}.out`), "utf8");
      if (got.err) { console.log(`FAIL: ${slug} (browser): X_eTaL stopped: ${got.err}`); failed++; }
      else if (got.out !== want) { console.log(`FAIL: ${slug} (browser): ${interactive ? "the transcript" : "the terminal"} differs from expected/play.out`); console.log(got.out); failed++; }
      else if (got.nb !== wantNb) { console.log(`FAIL: ${slug} (browser): the notebook differs from expected/${slug}.out`); failed++; }
      else if (interactive && got.controls) { console.log(`FAIL: ${slug} (browser): a game played by clicks shows New game and Restart (only Play belongs)`); failed++; }
      else if (readFileSync(join(dir, "play.xtl"), "utf8").includes("[]S_HOW") && !got.pics) { console.log(`FAIL: ${slug} (browser): play.xtl shows pictures but none appeared`); failed++; }
      else {
        const toml = readFileSync(join(dir, "game.toml"), "utf8");
        const wiki = (toml.match(/^wikipedia = "(.*?)"/m) || [])[1];
        const problem = (interactive && await boardProblems(page)) || await aboutProblems(page, ".brand h1 a.wiki, .brand h1 button.about-open", wiki);
        if (problem) { console.log(`FAIL: ${slug} (browser): ${problem}`); failed++; }
        else if (interactive) console.log(`ok: ${slug} (browser: ${typed.length} clicks on the map in its dialog, transcript and notebook match the goldens, closed by Escape, the background and the X, only Play, ${wiki ? "Wikipedia link" : "about dialog"})`);
        else console.log(`ok: ${slug} (browser: ${typed.length} lines typed, terminal and notebook match the goldens${got.pics ? `, ${got.pics} pictures` : ""}, ${wiki ? "Wikipedia link" : "about dialog"})`);
      }
    } catch (e) { console.log(`FAIL: ${slug} (browser): ${e.message}`); failed++; }
    await page.close();
  }
} catch (e) { console.log(`FAIL: browser test: ${e.message}`); failed++; }
if (process.env.BENCH_OUT) (await import("node:fs")).writeFileSync(process.env.BENCH_OUT, JSON.stringify(timings, null, 2));
console.log(`browser-test: ${games.length} game(s)${live ? ` at ${live}` : ""}${failed ? ", FAILURES" : ", all passed"}`);
await done(failed ? 1 : 0);
