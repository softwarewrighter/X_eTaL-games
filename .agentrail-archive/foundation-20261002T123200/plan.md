# foundation

Saga 1 of X_eTaL-games (docs/plan.md): the process, the vendored
interpreter, the game sub-project layout (with scripted-input
goldens), the live-site pipeline, and one game (horse-race) published
end to end.

Model: ../X_eTaL-demos (same scripts, adapted from demos/ to games/),
../X_eTaL (CHANGES.md).

Rules: game rules live in X_eTaL, never in the page; each game is its
own sub-project under games/<slug>/; X_eTaL only through the vendored
snapshot in vendor/xetal/; missing features and bugs go in
docs/xetal-asks.md; third-party assets are fetched by scripts, never
tracked. Every step: `just gate` passes, docs (README, CHANGES.md,
plan, asks) updated, .gitignore sane, a detailed commit to main
including .agentrail/, `agentrail complete`, push.

## Steps

1. scaffold -- process, CLAUDE.md/AGENTS.md, README, COPYRIGHT,
   LICENSE, CHANGES.md, justfile, gate, docs/plan.md,
   docs/xetal-asks.md.
2. vendor-xetal -- `just vendor [REF]`, vendor/xetal/VENDORED,
   `just xetal`, vendor probe, gate check.
3. game-layout -- games/_template, game.toml, golden runner with .in
   stdin, self-test, recipes.
4. pages-pipeline -- shared/microscope copied, catalog from game.toml,
   build-pages, pages.yml, deploy verified.
5. horse-race -- first game end to end (CLI, play.xtl, web), live.
