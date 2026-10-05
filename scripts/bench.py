#!/usr/bin/env python3
"""How fast the games run, and whether they got slower.

For every game: its scripted program (<slug>.xtl) and its terminal
session (play.xtl with expected/play.in) run by the vendored xetal,
the fastest of several runs (the least disturbed by the rest of the
machine), in milliseconds; and its page's scripted
session in headless Chrome (scripts/browser-test.mjs, BENCH_OUT).

  scripts/bench.py                 # measure and compare with bench/baseline.json
  scripts/bench.py --baseline      # measure and write bench/baseline.json
  scripts/bench.py --xetal PATH --out FILE   # measure another xetal build (no pages)

The comparison fails (exit 1) when a native measure (scripted or
terminal) is more than 15 % and more than 25 ms slower than the
baseline (page times are one run each, with the test's own pauses:
shown, not judged); it is skipped, with a note,
on a host other than the baseline's (timings are per machine).
"""
import json
import os
import socket
import statistics
import subprocess
import sys
import tempfile
import time
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BASELINE = ROOT / "bench" / "baseline.json"
RUNS = 7
SLOWER, FLOOR = 1.15, 25
GATED = ("script_ms", "play_ms")


def games():
    out = subprocess.run([str(ROOT / "scripts/games.py"), "list"], capture_output=True, text=True, check=True)
    return [s for s in out.stdout.split() if s]


def timed(cmd, cwd, stdin_file, env):
    times = []
    for _ in range(RUNS):
        with open(stdin_file or os.devnull) as inp:
            t = time.perf_counter()
            subprocess.run(cmd, cwd=cwd, stdin=inp, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env, check=False)
            times.append((time.perf_counter() - t) * 1000)
    return round(min(times))


def measure(xetal, pages):
    env = dict(os.environ, XETAL_PATH=str(ROOT / "lib"))
    results = {}
    for slug in games():
        d = ROOT / "games" / slug
        draw = tempfile.mkdtemp()
        run = [xetal, "run", "--seed", "1", "--draw", draw]
        r = {"script_ms": timed(run + [f"{slug}.xtl"], d, None, env)}
        if (d / "play.xtl").exists():
            r["play_ms"] = timed(run + ["play.xtl"], d, d / "expected" / "play.in", env)
        results[slug] = r
    if pages:
        out = tempfile.mktemp(suffix=".json")
        # The browser test's own results are shown (it is the gate's browser test too).
        subprocess.run([str(ROOT / "scripts/browser-test.mjs")], env=dict(os.environ, BENCH_OUT=out), check=True)
        for slug, ms in json.loads(Path(out).read_text()).items():
            results.setdefault(slug, {})["page_ms"] = ms
    return results


def xetal_build():
    return subprocess.run([str(ROOT / "scripts/build-xetal.sh")], capture_output=True, text=True, check=True).stdout.strip()


def table(results, base=None):
    lines = ["| Game | Scripted (ms) | Terminal (ms) | Page (ms) |", "| ---- | ------------- | ------------- | --------- |"]
    for slug, r in results.items():
        def cell(k):
            v = r.get(k)
            if v is None:
                return "-"
            b = (base or {}).get(slug, {}).get(k)
            return f"{v}" if b is None else f"{v} (was {b})"
        lines.append(f"| {slug} | {cell('script_ms')} | {cell('play_ms')} | {cell('page_ms')} |")
    return "\n".join(lines)


def main(args):
    if "--xetal" in args:
        xetal = args[args.index("--xetal") + 1]
        out = Path(args[args.index("--out") + 1])
        out.write_text(json.dumps(measure(xetal, pages=False), indent=2) + "\n")
        print(f"bench: wrote {out}")
        return 0
    vendored = tomllib.loads((ROOT / "vendor/xetal/VENDORED").read_text())["commit"][:7]
    results = measure(xetal_build(), pages=True)
    host = socket.gethostname().split(".")[0]
    if "--baseline" in args:
        BASELINE.write_text(json.dumps({"host": host, "xetal": vendored, "runs": RUNS,
                                        "date": time.strftime("%Y-%m-%d"), "results": results}, indent=2) + "\n")
        print(table(results))
        print(f"bench: baseline written ({host}, X_eTaL {vendored})")
        return 0
    base = json.loads(BASELINE.read_text())
    print(table(results, base["results"]))
    if base["host"] != host:
        print(f"bench: baseline is from {base['host']}, this is {host}: not compared")
        return 0
    # Only the native times (the fastest of several runs) can fail: a page
    # time is one run that includes the test's own pauses, so it is shown
    # for information.
    slow = [f"{slug} {k}: {v} ms (baseline {base['results'][slug][k]} ms)"
            for slug, r in results.items() for k, v in r.items()
            if k in GATED and k in base["results"].get(slug, {})
            and v > base["results"][slug][k] * SLOWER and v - base["results"][slug][k] > FLOOR]
    if slow:
        print("bench: slower than the baseline by more than 15 %:\n  " + "\n  ".join(slow))
        print("bench: if this is expected (a new X_eTaL, a bigger game), rewrite it: just bench --baseline")
        return 1
    print(f"bench: ok (no measure more than 15 % slower than the baseline of {base['date']}, X_eTaL {base['xetal']})")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
