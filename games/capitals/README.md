# Capitals

A dot marks a capital on the world map: name it from four choices,
the place and its three nearest neighbors. Choose a region (or the
whole world) and how many questions; the game says right or wrong,
how far the place you chose is, and keeps score.

The program reads its data from TOML: the coastlines and the 199
national capitals from Natural Earth (fetched, not tracked), and
places of your own from `places.toml`. It projects every coastline
point onto the map at once, draws the map, and finds the nearest
places by computing every distance at once.

Live: [the Capitals page](https://softwarewrighter.github.io/X_eTaL-games/capitals/)
-- the game (`play.xtl`) in a terminal, the scripted game
(`capitals.xtl`) as a notebook, and the sources: everything on the page
is X_eTaL's own output, run in your browser.

![The Capitals page](screenshot.png)

## The program

From `Atlas.xtl`, the rules library:

```
all := (("assets/cache/world.toml" "place") []T_ABLE ("places" "fields")) c_at ("places.toml" "place") []T_ABLE ("places" "fields")
l:c_ell := { la lo -> 1 + (cols * f_loor 2 * 84 - la) + f_loor 2 * 180 + lo }
l:c_lose := { i ->
  r := (p_i @) / 180
  ((s_in r * i s_elect lat) * s_in r * lat) + (c_os r * i s_elect lat) * (c_os r * lat) * c_os r * lon - i s_elect lon
}
```

## How it works

Read right to left.

- `[]T_ABLE` reads a table of tables from a TOML file as a matrix of
  texts, one row per place (name, latitude, longitude, region); the
  capitals and your own places are joined with `c_at`. The numbers are
  texts in TOML, so each is read with `n_umbers`.
- `l:c_ell` is the projection: the plate carree, a scale and a shift of
  each coordinate, onto half-degree cells (288 rows from 84 N to 60 S,
  720 columns). It takes every point at once.
- The map: all 21615 coastline points projected, the cells sorted
  without repeats, and each cell's gap of 0s laid out by one
  `r_eplicate`, then reshaped to 288 by 720. `[]G_RID` draws a matrix
  this large as an image, a pixel per cell.
- `l:c_lose` gives the cosine of the angle between one place and every
  place at once (the spherical law of cosines); `g_rade` of its
  negation orders them nearest first, and the three nearest in the
  game's region are the wrong choices. `l:k_m` turns the cosine into
  kilometers.
- `l:d_ot` adds the asked place as a 7 by 7 dot: an outer product
  (`'& t_able`) of the rows and the columns near it.

## Play it

```bash
just play capitals       # at the terminal
just run capitals        # the scripted game
just show capitals       # the scripted game as a notebook
just repl capitals       # then "a:" u_se< "Atlas"
just test-game capitals  # compare with the expected output
```

## Adding places

Add places of your own to `places.toml`: list each key in `places`
and give it a table with its name, latitude, longitude (degrees, south
and west negative) and region. The file's comment shows an example,
and `test.sh` checks that added places join the quiz. They are asked
like the capitals, with the nearest places in their region as the
wrong choices. The regions menu is the `regions` list in the same
file.

## Assets

`assets/fetch.sh` downloads Natural Earth's 1:110m land and populated
places and 1:50m countries (public domain, pinned to a commit and
checked by SHA-256) into the git-ignored `assets/cache/`, and
`assets/convert.py` writes `assets/cache/world.toml` from them: the
coastline points, and each national capital's name, latitude,
longitude and its country's UN region. Nothing is projected there.

## Workarounds

All named in [`docs/xetal-asks.md`](../../docs/xetal-asks.md):

- Numbers are strings in TOML (X_eTaL's data-only `.xtln` would carry
  them as numbers), so each is read with `n_umbers`, one short string
  at a time: on long text it is slow.
- The map's cells are marked by sorting and one `r_eplicate`, since
  `m_ember?` is slow on long vectors.
- The data files are named relative to the game's directory, where
  every recipe runs the programs: `[]T_ABLE` reads paths relative to
  the working directory, not to the file that names them.
