# Shared libraries

X_eTaL libraries the games share, so no game copies a helper from
another. A game imports one under a short alias, `"b:" u_se< "Board"`;
the scripts find them through `XETAL_PATH` (set to this directory by
`just run`, `just play`, `just repl` and the tests), and a game's page
loads the ones it lists (`shared` in its `page::Game`).

| Library | What it gives |
| ------- | ------------- |
| `Play.xtl` | reading what a player types: the numbers on a line (`l:n_umbers`), the first number or -1 (`l:n_umber`), which of some letters a line starts with (`l:l_etter`) |
| `Text.xtl` | printing a strand of lines (`l:l_ines`); which rows of a text table hold in a state (`l:h_olds`) |
| `Board.xtl` | square numbers and rows/columns both ways (`l:c_ell`, `l:r_c`); every square's neighbor count, edges not wrapping (`l:a_round`); the plus of four neighbors (`l:p_lus`); a character board spaced as the games print it (`l:s_paced`) |
| `State.xtl` | several items of a state vector changed at once (`l:u_pdate`); a stretch replaced (`l:p_ut`) |

`test.xtl` exercises every function on small cases;
`scripts/test-lib.sh` (in the gate) checks it and each library's
exported types against `expected/`. `docs/style.md` says when a helper
belongs here: as soon as a second game needs it.
