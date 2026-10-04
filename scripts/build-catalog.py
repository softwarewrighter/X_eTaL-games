#!/usr/bin/env python3
"""Write pages/index.html: the live catalog of the games.

  scripts/build-catalog.py [OUT]     # default pages/index.html

One card per game (scripts/games.py json, catalog order): title,
the lesson, summary, concepts, status, a link to its live page (pages/<slug>/, when
it has a web app) and to its README. The footer is the X_eTaL live
demo's: copyright, license, the repository, and the build's provenance
build (host, this repo's sha, yyyymmddThhmmss), plus the vendored X_eTaL commit.
"""
import datetime
import html
import json
import socket
import subprocess
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
REPO = "https://github.com/softwarewrighter/X_eTaL-games"
XETAL = "https://github.com/softwarewrighter/X_eTaL"
STATUS = {"live": "Live", "draft": "In progress", "deferred": "Waiting on X_eTaL"}

PAGE = """<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>X_eTaL Games</title>
<link rel="icon" href="favicon.ico">
<style>
:root {{ --bg:#fbfaf7; --fg:#1d1d1f; --muted:#5f6368; --card:#ffffff; --line:#e3e0d8;
  --accent:#2457c5; --chip:#eef2fb; --live:#1f7a3a; --draft:#9a6200; --deferred:#8a8a8a; }}
@media (prefers-color-scheme: dark) {{ :root:not([data-theme="light"]) {{
  --bg:#141518; --fg:#e8e6e3; --muted:#a0a4ab; --card:#1d1f23; --line:#30333a;
  --accent:#8fb0ff; --chip:#262b36; --live:#5fcf7f; --draft:#e0a84a; --deferred:#8d9097; }} }}
:root[data-theme="dark"] {{ --bg:#141518; --fg:#e8e6e3; --muted:#a0a4ab; --card:#1d1f23;
  --line:#30333a; --accent:#8fb0ff; --chip:#262b36; --live:#5fcf7f; --draft:#e0a84a; --deferred:#8d9097; }}
* {{ box-sizing: border-box; }}
body {{ margin:0; background:var(--bg); color:var(--fg);
  font: 16px/1.5 system-ui, -apple-system, "Segoe UI", sans-serif; }}
main, footer {{ max-width: 980px; margin: 0 auto; padding: 0 16px; }}
header {{ padding: 48px 0 24px; }}
h1 {{ font-size: 2rem; margin: 0 0 8px; letter-spacing: -0.01em; }}
.lede {{ color: var(--muted); max-width: 46rem; margin: 0; }}
.lede a, footer a {{ color: var(--accent); }}
.grid {{ display:grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap:16px; padding: 8px 0 40px; }}
.card {{ background:var(--card); border:1px solid var(--line); border-radius:12px; padding:18px;
  display:flex; flex-direction:column; gap:10px; }}
.card h2 {{ font-size:1.15rem; margin:0; }}
.shot img {{ width:100%; aspect-ratio: 13 / 9; object-fit: cover; object-position: top; border-radius:8px; border:1px solid var(--line); display:block; }}
.card p {{ margin:0; color:var(--muted); }}
.card p.lesson {{ color:var(--fg); font-weight:600; font-size:.9rem; }}
.status {{ font-size:.8rem; font-weight:600; }}
.status.live {{ color:var(--live); }} .status.draft {{ color:var(--draft); }} .status.deferred {{ color:var(--deferred); }}
.chips {{ display:flex; flex-wrap:wrap; gap:6px; }}
.chip {{ background:var(--chip); border-radius:999px; padding:2px 10px; font-size:.8rem; }}
.links {{ margin-top:auto; display:flex; gap:16px; font-weight:600; }}
.links a {{ color:var(--accent); text-decoration:none; }}
.links a:hover {{ text-decoration:underline; }}
.card h2 a.wiki {{ color: inherit; text-decoration: none; }}
.card h2 a.wiki:hover {{ text-decoration: underline; }}
.card h2 .glyph {{ color: var(--muted); vertical-align: 0.05em; }}
.card h2 button.about-open {{ font: inherit; color: inherit; background: none; border: 0; padding: 0; cursor: pointer;
  text-align: left; text-decoration: underline dotted; text-underline-offset: 0.2em; }}
dialog.about {{ border: 1px solid var(--line); border-radius: 12px; padding: 0; max-width: min(36rem, 92vw);
  background: var(--card); color: var(--fg); }}
dialog.about::backdrop {{ background: rgba(0, 0, 0, 0.45); }}
dialog.about .about-box {{ position: relative; padding: 18px 22px 14px; }}
dialog.about h2 {{ margin: 0 0 8px; font-size: 1.15rem; }}
dialog.about p {{ margin: 0 0 6px; line-height: 1.55; color: var(--fg); }}
dialog.about .close {{ position: absolute; top: 6px; right: 8px; font: 1.4rem/1 system-ui, sans-serif;
  background: none; border: 0; color: var(--muted); cursor: pointer; padding: 4px 8px; }}
.empty {{ color:var(--muted); padding: 24px 0 48px; }}
.idioms {{ padding: 0 0 24px; }}
.idioms h2 {{ font-size: 1.2rem; margin: 0 0 6px; }}
.idioms table {{ border-collapse: collapse; width: 100%; margin-top: 10px; font-size: .95rem; }}
.idioms th, .idioms td {{ text-align: left; padding: 6px 10px; border-bottom: 1px solid var(--line); }}
.idioms th {{ color: var(--muted); font-weight: 600; }}
.idioms a {{ color: var(--accent); }}
footer {{ border-top:1px solid var(--line); padding-top:16px; padding-bottom:32px; color:var(--muted); font-size:.85rem; }}
footer .sep {{ margin: 0 8px; }}
.brand {{ display:flex; align-items:center; gap:16px; margin-bottom: 8px; }}
.brand h1 {{ margin: 0; }}
.logo {{ height: 56px; width: auto; border-radius: 8px; }}
code {{ font-family: ui-monospace, "JuliaMono", Menlo, monospace; }}
</style>
</head>
<body>
<main>
<header>
<div class="brand"><img class="logo" src="modern-xetal-logo.jpg" alt="X_eTaL"><h1>Games</h1></div>
<p class="lede">Small games written in <a href="{xetal}">X_eTaL</a>, a typed array language.
Each one teaches one array-programming idea and happens to be playable. A game's page runs
it as X_eTaL runs it, in your browser: the game in a terminal, its scripted program as a
notebook, and the source; everything on it is X_eTaL's own output.</p>
</header>
{body}
</main>
<footer>
<span>Copyright (c) 2026 Michael A Wright</span><span class="sep">&middot;</span>
<span>MIT License</span><span class="sep">&middot;</span>
<a href="{repo}" target="_blank">Repository</a><span class="sep">&middot;</span>
<span>X_eTaL <a href="{xetal}/commit/{xsha}" target="_blank">{xshort}</a></span><span class="sep">&middot;</span>
<span>build (host {host}, sha {commit}, {stamp})</span>
</footer>
<script>
// A title without a Wikipedia article opens a dialog: Escape closes it
// (the browser does), as do a click outside it and its X.
for (const b of document.querySelectorAll("button.about-open")) {{
  const d = document.getElementById(b.dataset.dialog);
  b.addEventListener("click", () => d.showModal());
  d.addEventListener("click", (e) => {{ if (e.target === d) d.close(); }});
  d.querySelector(".close").addEventListener("click", () => d.close());
}}
</script>
</body>
</html>
"""


GLYPH = ('<svg class="glyph" viewBox="0 0 16 16" width="14" height="14" aria-hidden="true">'
         '<path d="M3 1.5h6.5L13 5v9.5H3z" fill="none" stroke="currentColor" stroke-width="1.3" stroke-linejoin="round"/>'
         '<path d="M9.5 1.5V5H13M5.5 8h5M5.5 10.5h5M5.5 13h3" fill="none" stroke="currentColor" stroke-width="1.1"/></svg>')


def title(m):
    """A game's title: a link to its Wikipedia article (new tab, page
    glyph), or a button opening a dialog on its history and play."""
    t = html.escape(m["title"])
    if m.get("wikipedia"):
        return (f'<a class="wiki" href="{html.escape(m["wikipedia"])}" target="_blank" '
                f'rel="noopener noreferrer" title="Wikipedia (opens in a new tab)">{t} {GLYPH}</a>')
    return (f'<button class="about-open" data-dialog="about-{html.escape(m["slug"])}" title="About this game">{t}</button>'
            f'<dialog class="about" id="about-{html.escape(m["slug"])}"><div class="about-box">'
            f'<button class="close" aria-label="Close">&times;</button><h2>{t}</h2>'
            f'<p>{html.escape(m.get("about", ""))}</p></div></dialog>')


def card(m):
    slug = html.escape(m["slug"])
    links = []
    if m.get("web"):
        links.append(f'<a href="{slug}/">Play</a>')
    links.append(f'<a href="{REPO}/tree/main/games/{slug}#readme">How it works</a>')
    chips = "".join(f'<span class="chip">{html.escape(c)}</span>' for c in m["concepts"])
    st = m["status"]
    pic = ""
    if m.get("web") and m.get("picture"):
        alt = html.escape(m["title"])
        pic = f'<a class="shot" href="{slug}/"><img src="{slug}/screenshot.png" alt="{alt}" loading="lazy"></a>\n'
    return (f'<article class="card" id="{slug}">\n{pic}'
            f'<span class="status {st}">{STATUS[st]}</span>\n'
            f'<h2>{title(m)}</h2>\n'
            f'<p class="lesson">{html.escape(m["lesson"])}</p>\n'
            f'<p>{html.escape(m["summary"])}</p>\n'
            f'<div class="chips">{chips}</div>\n'
            f'<div class="links">{" ".join(links)}</div>\n</article>')


def git(*args):
    r = subprocess.run(["git", "-C", str(ROOT), *args], capture_output=True, text=True)
    return r.stdout.strip() or "unknown"


def main():
    out = Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT / "pages" / "index.html"
    games = json.loads(subprocess.run([str(ROOT / "scripts" / "games.py"), "json"],
                                      capture_output=True, text=True, check=True).stdout)
    vend = tomllib.loads((ROOT / "vendor" / "xetal" / "VENDORED").read_text())
    if games:
        rows = "".join(f'<tr><td>{html.escape(m["lesson"])}</td><td><a href="#{html.escape(m["slug"])}">{html.escape(m["title"])}</a></td></tr>'
                       for m in games if m["status"] == "live")
        idioms = ('<section class="idioms"><h2>One array idea per game</h2>\n'
                  '<p class="lede">Different games, the same few array ideas: each game is written around one.</p>\n'
                  f'<table><thead><tr><th>The array idiom</th><th>Game</th></tr></thead><tbody>{rows}</tbody></table></section>\n')
        body = idioms + '<section class="grid">\n' + "\n".join(card(m) for m in games) + "\n</section>"
    else:
        body = '<p class="empty">The first game is on its way.</p>'
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(PAGE.format(
        body=body, repo=REPO, xetal=XETAL, commit=git("rev-parse", "--short", "HEAD"),
        xsha=vend["commit"], xshort=vend["commit"][:7], host=socket.gethostname().split(".")[0],
        stamp=datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%S")))
    print(f"catalog: {out} ({len(games)} game(s))")


if __name__ == "__main__":
    main()
