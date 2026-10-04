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

const slugs = process.argv.slice(2).length ? process.argv.slice(2)
  : execFileSync(join(root, "scripts/games.py"), ["list"], { encoding: "utf8" }).split("\n").filter(Boolean);
const games = slugs.filter((s) => existsSync(join(root, "games", s, "web/Cargo.toml"))
  && existsSync(join(root, "games", s, "expected/play.in")));

const server = spawn(join(root, "scripts/serve-pages.sh"), [String(webPort)], { stdio: "ignore" });
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
    try { return await (await fetch(`http://127.0.0.1:${cdpPort}${path}`, { method })).json(); }
    catch { await sleep(100); }
  }
  throw new Error("Chrome did not answer on the DevTools port");
}

// One page: a DevTools session with a promise per command.
async function open(url) {
  const target = await json(`/json/new?${encodeURIComponent(url)}`, "PUT");
  const ws = new WebSocket(target.webSocketDebuggerUrl);
  await new Promise((r, e) => { ws.onopen = r; ws.onerror = e; });
  let id = 0; const waiting = new Map();
  ws.onmessage = (m) => { const msg = JSON.parse(m.data); waiting.get(msg.id)?.(msg); waiting.delete(msg.id); };
  const send = (method, params = {}) => new Promise((r) => { waiting.set(++id, r); ws.send(JSON.stringify({ id, method, params })); });
  const evaluate = async (expression) => {
    const r = await send("Runtime.evaluate", { expression, awaitPromise: true, returnByValue: true });
    if (r.result?.exceptionDetails) throw new Error(r.result.exceptionDetails.exception?.description || "page error");
    return r.result?.result?.value;
  };
  const key = async (k, code) => {
    for (const type of ["rawKeyDown", "keyUp"]) await send("Input.dispatchKeyEvent", { type, key: k, code: k, windowsVirtualKeyCode: code, nativeVirtualKeyCode: code });
  };
  return { evaluate, key, close: () => ws.close() };
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
async function aboutProblems(page, sel, wiki) {
  const a = await page.evaluate(ABOUT(sel));
  if (wiki) {
    if (a.kind !== "wiki" || a.href !== wiki || a.target !== "_blank" || !a.glyph) return `title should link to ${wiki} in a new tab with a glyph (got ${JSON.stringify(a)})`;
    return "";
  }
  if (a.kind !== "dialog" || a.text < 40) return `title should open an about dialog (got ${JSON.stringify(a)})`;
  for (const [how, close] of [["Escape", null], ["a click outside", BACKDROP], ["the X", XBUTTON]]) {
    if (!(await page.evaluate(OPEN(sel)))) return "the dialog did not open";
    if (close) await page.evaluate(close); else { await page.key("Escape", 27); await sleep(50); }
    if (await page.evaluate(IS_OPEN)) return `${how} did not close the dialog`;
  }
  return "";
}

let failed = 0;
try {
  await json("/json/version");
  for (const slug of games) {
    const dir = join(root, "games", slug);
    const typed = readFileSync(join(dir, "expected/play.in"), "utf8").split("\n").filter((l, i, a) => i < a.length - 1 || l);
    const page = await open(`http://127.0.0.1:${webPort}/X_eTaL-games/${slug}/?seed=1`);
    try {
      const got = await page.evaluate(PLAY(typed));
      const want = readFileSync(join(dir, "expected/play.out"), "utf8");
      const wantNb = readFileSync(join(dir, `expected/${slug}.out`), "utf8");
      if (got.err) { console.log(`FAIL: ${slug} (browser): X_eTaL stopped: ${got.err}`); failed++; }
      else if (got.out !== want) { console.log(`FAIL: ${slug} (browser): the terminal differs from expected/play.out`); console.log(got.out); failed++; }
      else if (got.nb !== wantNb) { console.log(`FAIL: ${slug} (browser): the notebook differs from expected/${slug}.out`); failed++; }
      else if (readFileSync(join(dir, "play.xtl"), "utf8").includes("[]S_HOW") && !got.pics) { console.log(`FAIL: ${slug} (browser): play.xtl shows pictures but none appeared`); failed++; }
      else {
        const toml = readFileSync(join(dir, "game.toml"), "utf8");
        const wiki = (toml.match(/^wikipedia = "(.*?)"/m) || [])[1];
        const problem = await aboutProblems(page, ".brand h1 a.wiki, .brand h1 button.about-open", wiki);
        if (problem) { console.log(`FAIL: ${slug} (browser): ${problem}`); failed++; }
        else console.log(`ok: ${slug} (browser: ${typed.length} lines typed, terminal and notebook match the goldens${got.pics ? `, ${got.pics} pictures` : ""}, ${wiki ? "Wikipedia link" : "about dialog"})`);
      }
    } catch (e) { console.log(`FAIL: ${slug} (browser): ${e.message}`); failed++; }
    page.close();
  }
  // The catalog: every card's title, the same way.
  const cat = await open(`http://127.0.0.1:${webPort}/X_eTaL-games/`);
  let catProblems = 0;
  for (const slug of slugs) {
    const toml = readFileSync(join(root, "games", slug, "game.toml"), "utf8");
    const wiki = (toml.match(/^wikipedia = "(.*?)"/m) || [])[1];
    const problem = await aboutProblems(cat, `[id="${slug}"] h2 a.wiki, [id="${slug}"] h2 button.about-open`, wiki);
    if (problem) { console.log(`FAIL: catalog ${slug}: ${problem}`); failed++; catProblems++; }
  }
  if (!catProblems) console.log(`ok: catalog (${slugs.length} titles: Wikipedia links and about dialogs)`);
  cat.close();
} catch (e) { console.log(`FAIL: browser test: ${e.message}`); failed++; }
console.log(`browser-test: ${games.length} game(s)${failed ? ", FAILURES" : ", all passed"}`);
await done(failed ? 1 : 0);
