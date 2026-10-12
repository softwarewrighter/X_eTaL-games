# Tower defense

Stop the creeps on the road before they get out. Pick a tower on the
toolbar (or `1`, `2`, `3`; left and right step through them) and click
a grass tile to build it there; click a tower to upgrade it (three
levels each). **Next wave** (or the space bar) sends the next wave when
you are ready; **Speed** (or `f`) runs the game 1, 2 or 4 times as fast;
the **Grimoire** (or `g`) shows every tower and creep, turning, with
what it does and what its upgrade costs. Six waves to beat, 20 lives; a
creep that gets out costs lives. The line under the top says what your
last click did (built, upgraded, or what is missing).

| Tower | Does |
| ----- | ---- |
| Arrow (40) | quick single shots; upgrades reach farther and shoot faster |
| Cannon (60) | slow shots that hurt every creep near where they land |
| Frost (50) | little damage, but slows the creeps it hits (and, upgraded, those near them) |

| Creep | Is |
| ----- | -- |
| Runner | fast and frail |
| Brute | slow, many hit points, costs 3 lives if it gets out |
| Shield | armored: each hit does its damage less 4 (never below 1); cannons crack it |

Live: [the Tower defense page](https://softwarewrighter.github.io/X_eTaL-games/tower-defense/)
-- Play opens the game in a dialog (close it with Escape, its X or a
click outside) and starts the clock;
[this link](https://softwarewrighter.github.io/X_eTaL-games/tower-defense/#play)
opens it at once. When the game ends, Play or any key starts a new one.

![Tower defense](screenshot.png)

## How it works

- **2D underneath, 2.5D on screen.** The game is a grid of tiles and a
  road; the picture is an isometric god's-eye view: every point of the
  board is turned into the picture by one linear map (`OX + TW * x - y`,
  `OY + TH * x + y`), and a click is turned back into a tile by its
  inverse. A click on a tower's box, which stands up over the tiles
  behind it, counts for the tower: every tower's box is tested at once.
- **The road is found by a flood.** Every road tile's distance to E,
  each one more than its nearest road neighbor, all at once until
  nothing changes; sorted farthest first, the tiles are the way along
  the road. A creep is a number (how far it has come), and its place on
  the board is worked out from that for every creep at once.
- **Every tower against every creep is one table** (towers down, creeps
  across): which creeps each tower reaches, and for each ready tower the
  one in reach that has come farthest. A shot is spawned for each.
- **Every hitting shot against every creep is another table**: the
  target, and with a splash every creep near where the shot lands,
  takes its damage less its armor (never below 1); a frost hit slows
  them. Built on [`lib/Ecs`](../../lib/README.md): creeps, towers and
  shots are rows of one world, spawned and despawned by masks.
- **The game is data.** `td.toml` holds the map (the road is wherever
  `=` is), the towers' numbers at each level, the creeps', the waves and
  the start.
- **The state is a tuple:** `(world, money, lives, waves begun, creeps
  to come, seconds to the next, tower picked, view, grimoire page,
  seconds played, standing, speed, message)`.
- Ticks, clicks and keys come to `play.xtl` as `[]E_VENT` events
  (`tick 0.05`, `click 384 192`, `key space`), on the page as at the
  command line.

## The pieces' art

The towers and creeps are drawn by the game's own X_eTaL as boxes and
balls for now. Their 3D models are asked of
[X_eTaL-extensions](https://github.com/softwarewrighter/X_eTaL-extensions)
(its Blender extension, driven by an X_eTaL program): see
[docs/extensions-ask.md](../../docs/extensions-ask.md). When they are
published, `assets/fetch.sh` will fetch them (never committed here),
`td.toml`'s `art` switch turns them on, and the board will also be
drawn as a three.js scene that the X_eTaL program describes.

## Play it

```bash
just show tower-defense       # the scripted game as a notebook
just play tower-defense       # at the terminal: lines such as tick 0.05, click 384 192, key space
just test-game tower-defense  # compare with the expected output
```

At the terminal nothing moves until you type a `tick` line; the page
gives the ticks.

## Known X_eTaL issue

The grass is kept as 1s and 0s made where the map is read: adding 0 to
a named true/false value stops with a type mismatch (the ask in
[docs/xetal-asks.md](../../docs/xetal-asks.md) about a comparison being
a number in one place and Bool in another).

## Assets

None yet (see The pieces' art).
