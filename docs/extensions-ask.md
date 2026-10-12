# Ask for X_eTaL-extensions: tower defense art

From X_eTaL-games (the tower defense game, `games/tower-defense/`) to
X_eTaL-extensions (the `blender` extension). The user's plan,
2026-10-11: the extensions repository draws the pieces (an X_eTaL
program driving Blender), publishes what it makes, and the game here
only uses the published files. Each side links to the other's part.

## What to make

Twelve models: three towers at three upgrade levels each, and three
creeps. Each model faces -Y (toward the camera at turntable frame 0),
stands on Z = 0, and is sized against one map tile (1 by 1 Blender
unit; a tower's base fills about 0.8 of the tile, a creep about 0.5).
A simple, chunky, readable look suits the game better than detail: the
map shows them about 64 pixels across. The game's ground colors, for
contrast: grass `#3a7d44`, road `#8d7b5e`.

### Towers

| Piece | Level 1 | Level 2 | Level 3 |
| ----- | ------- | ------- | ------- |
| `arrow` (fast, single target) | a wooden post tower (square, plank-brown) with a small crossbow on top | a stone tower (gray, crenellated top) with twin crossbows | a taller stone tower with a repeating ballista and a red banner |
| `cannon` (slow, splash damage) | a squat round stone base with a short black barrel | a wider base with a longer barrel and iron bands | a turret with twin barrels on a turning ring, brass trim |
| `frost` (slows creeps) | a small pedestal holding one pale-blue ice crystal (a stretched octahedron) | a taller pedestal with a crystal cluster and a silver ring | a crystal floating above the pedestal, shards around it, glowing (emission) |

The crossbows, barrels and crystal are the part that aims: on the map
the game turns the tower toward its target (see the map sheet below),
so the aiming part should point -Y like the rest.

### Creeps

| Piece | Look | In the game |
| ----- | ---- | ----------- |
| `runner` | a small low wedge on four short legs, bright green, two round eyes | fast, few hit points |
| `brute` | a big blocky body with two horns, red-brown, heavy feet | slow, many hit points |
| `shield` | a dome shell (half sphere) with armor plates, steel blue | armored: each hit does less |

## The files, and their format

The game is 2D underneath (a grid of tiles, a road, creeps moving
along it) and shown in 2.5D: a fixed isometric god's-eye view, as in
a classic real-time strategy game. Its X_eTaL program writes the board
as a scene (which model, where, facing which way, which animation at
what time, the camera), and the game page renders that scene with
three.js (the user's choice, 2026-10-11): the page draws only what the
program describes, as it does with the program's SVG. So the main
format is glTF; still pictures are a fallback.

For each of the twelve pieces (names: `arrow-1`, `arrow-2`, `arrow-3`,
`cannon-1` ... `frost-3`, `runner`, `brute`, `shield`):

| File | What |
| ---- | ---- |
| `PIECE.glb` | the model, binary glTF 2.0, for three.js's GLTFLoader: meters as Blender units (one tile is 1 by 1), Y up as glTF has it (Blender's exporter turns Z up into Y up), facing -Z in glTF (the -Y front above), its origin at the middle of its base; materials as glTF PBR (base color, emission for the frost glow); under about 3000 triangles; no textures needed (flat colors), or small embedded ones |
| animations in it | named clips the program can ask for by name: every piece `idle` (a loop, 2 seconds: a tower's banner or crystal moving, a creep breathing); creeps `walk` (a loop, 1 second, feet moving in place: the game moves the creep); towers `fire` (once, 0.4 seconds: a recoil, a flash); creeps `hit` (once, 0.3 seconds) |
| `PIECE-turn.png` | a fallback for the grimoire without 3D: 16 frames, the model turned 22.5 degrees counterclockwise (seen from above) each frame, frame 0 its front, the camera 20 degrees above the horizon, transparent background; 16 frames of 256 by 256 in a 4 by 4 sheet (1024 by 1024), row by row |
| `PIECE-map.png` | a fallback for the board without 3D: 8 frames, the model facing east, then each 45 degrees turned clockwise on screen (east, southeast, south, southwest, west, northwest, north, northeast); an orthographic camera 35 degrees above the horizon (the isometric look), one tile 64 pixels wide, transparent background; 8 frames of 96 by 96 in one row (768 by 96) |
| `PIECE-mesh.toml` (optional) | the model as data X_eTaL can read (it reads TOML, not binary glTF), for a renderer written in X_eTaL itself (projection as one matrix product, flat shading from face normals, faces drawn far to near): a low-poly version (a few hundred triangles), `vertices` (x y z per vertex, as strings of numbers, Z up), `faces` (three vertex numbers each, from 1), `colors` (a color per face, `#rrggbb`); a mesh per animation pose is welcome but not needed |
| `manifest.toml` | the pieces, their files, the animation clips and their lengths, the PNG frame sizes and counts, each file's SHA-256, the license, and the program and commit that made them |

PNG with alpha, 8 bits a channel. Everything together under about
10 MB. If the PNG fallbacks are much extra work, the `.glb` files come
first: the game draws its own shapes where neither is there.

## Where

Published on the extensions' live site, for example
`https://softwarewrighter.github.io/X_eTaL-extensions/blender/td/`
(from `pages/blender/td/`), with `manifest.toml` beside the sheets.
Tell us the address and a commit; the game's
`games/tower-defense/assets/fetch.sh` pins every file by its SHA-256 and
fetches into the git-ignored `assets/cache/`, as the game fetches all
outside files (they are never committed here). The pages build copies
them next to the game's page.

## Credit and links

The game's README and its grimoire name the program that made the
models (its path in X_eTaL-extensions) and link to it; your demo's
README can link to the game's page. Please state the files' license in
`manifest.toml` (we assume the extensions repository's own).

## Until then

The game is built now with its own shapes, drawn by its X_eTaL as an
isometric SVG board; the three.js view comes next, with placeholder
shapes. Its data file says whether the art is in use; when the files
are published, `fetch.sh` gets their checksums and the flag turns on,
in one commit, with no change to the rules.

## What the extension may need

As we read `extensions/blender/lib/Bpy.xtl` (cubes, labels, a ground, an
orbit camera, stills, video, exports), the models above need more
shapes (cylinders, cones, spheres, a torus, a profile turned into a
solid), materials with emission, keyframed actions exported as named
glTF animations, and, for the fallbacks, a camera keyed to stills at
chosen angles (the turntable as 16 stills, the map as 8) and the frames
joined into a sheet. Those are yours to design; the list is only what the
pieces above call for.
