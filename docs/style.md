# How the games are written

The house style for game code in this repository: how a game's X_eTaL
should read. It applies to every game's rules library, scripted game
and terminal game, and to the shared libraries in `lib/`. It is about
the X_eTaL at the known-good commit (`XETAL_COMMIT`); features
X_eTaL has not shipped yet are listed at the end, with what to do in
the meantime.

The aim: a reader who knows a little X_eTaL sees the rules of the game,
stated as operations on whole arrays, in a few named pieces. Shorter is
better when it is clearer, never when it is cryptic.

## Arrays first

- State the rule for the whole board, all the pieces or all the moves
  at once. A loop over squares, robots or moves (recursion that walks
  an index) is a sign the array form has not been found yet.
- Prefer one table (`t_able`), one inner product (`i_nner`), one
  selection (`s_elect`) or one scan (`s_\`) over a sequence of steps.
- Recursion is for what is sequential by nature: turns of a game, a
  fixed point (a flood fill), a search (minimax). Even then, each step
  should be an array operation.
- A random choice among many is one roll over all of them (a random
  order is `g_rade` of random keys; its inverse is `g_rade` again).

## Small named pieces

- Give a sub-result a name when it has a meaning in the game
  (`crash`, `fits`, `place`), not to shorten a line.
- A small point-free piece is a train: a fork `[f g h] y` is
  `(f y) g (h y)` (the mean is `['+ r_/ / t_ally]`), an atop `[f g] y`
  is `f (g y)`. Name it like a function, `l:m_ean := ['+ r_/ / t_ally]`.
  Use one when it reads as a phrase; write a lambda when it does not.
- A function does one thing of the game; the rules library reads as a
  list of the game's rules.

## Idioms the games use

- The first condition that holds, as data: `f_irst (conds r_eplicate
  codes) c_at 0` picks a status, a refusal or a message, instead of a
  chain of guards.
- A weighted rating or a count per place is an inner product:
  `weights '+ '* i_nner features`, `((r_ange 256) '= t_able k) '+ '*
  i_nner w`.
- A sign is a dyadic fork, `d [> - <] 0`; a dyadic fork is written
  inline (a named train is monadic).
- A sequence of steps is a train of atops, `[f [g h]]` (f of g of h).
- An optional transform is a power of 0 or 1, `(1 * cond) 'f p_ower x`
  (a power count must be a number, not a Bool).
- A function over the rows of a matrix: `m_ap` over the row numbers
  (`'{ f _r s_elect m } m_ap r_ange t_ally m`); `m_ap` alone maps items.
- What a scripted game guarantees is stated with X_eTaL's system macro
  `a_ssert<`, bound so it prints nothing: `ok := "3 = t:s_tatus end"
  a_ssert< "perfect play is a draw"`. It is silent when the condition
  holds and writes the condition as written to standard error when it
  does not, which fails the tests (the self-test proves it). The control
  macros (`i_f<`, `u_nless<`) are not used: a guard does their job.
- A message with values in it is a format string, `@ f_ormat< "SCORE
  {t:s_core s}"`: X_eTaL's system macro, whose template is checked when
  the program is compiled (an unclosed `{` stops it), not a chain of
  `"..." c_at (f_ormat x) c_at "..."`. Text inside `{}` is written as
  it is; `{{` and `}}` are braces. `c_at` stays for joining arrays.

## Libraries

- Anything two games need is in a shared library in `lib/` (`Play`
  for terminal input, `Board` for grids, `Text` for printed text,
  `State` for updates), imported under a short alias. Never copy a
  helper from one game to another.
- A game's rules are one library, `games/<slug>/<Name>.xtl`, named
  unlike the game (on a case-insensitive disk `Guess.xtl` is
  `guess.xtl`); exports are `l:`, private helpers have no prefix.
- A library holds definitions only (no top-level expressions).

## Comments and the cross-reference

Comments follow X_eTaL's convention (lang-choices S9), so `xetal doc`
can show them: `##` is documentation (the block at the top of a file
documents the file; a block directly above a definition documents it),
`###` is a section heading, `#` is an ordinary note it leaves out. A
`# ::` type line goes above a definition's `##` block, never between
the block and the definition. Every export of a library has a `##`
block (`scripts/check-docs.py`, in the gate), and the shared libraries
show `## >>` examples with their output, which the gate runs (`xetal
doc --test`). The cross-reference is part of the live site
(`pages/doc/`, `scripts/build-docs.sh`), linked from the catalog and
from each game page.

## Types and names

- Every export has a type comment above it, `# :: Int -> Int -> Int`,
  exactly what X_eTaL infers: the library's golden
  (`expected/<Name>.out`) lists the inferred types, and
  `scripts/check-types.py` (in the gate) checks every comment against
  it, in any library that has type comments.
- Status, event and command codes are named once as constants
  (`l:won := 1`) and compared by name, until X_eTaL has enums.
- Game state that is more than one array is a flat vector read by
  named accessors (`l:r_obots s`), until X_eTaL has records.

## Reading right to left

X_eTaL reads right to left with no precedence. The traps the games
fell into, all caught by the goldens:

- `f s = 1 ? ...` is `f (s = 1)`: put the constant on the left,
  `1 = f s ? ...`.
- `(p - 1 + n)` is `p - (1 + n)`: parenthesize, `((p - 1) + n)`.
- `a * b + c` multiplies by `b + c`: `(a * b) + c`.
- A function's argument is everything to its right: `f x c_at y` is
  `f (x c_at y)`.
- `7 m_od 3` is 1: the dividend is on the left.
- A strand joins only literals: `1 x 2` is an error; write
  `1 c_at x c_at 2`.

## Pictures and output

- A board is shown as text the program prints, and, when it reads
  better, as a picture X_eTaL draws (`[]S_HOW []G_RID board`); the page
  shows nothing else (`docs/plan.md` A11).
- Pictures suit character and 0/1 boards; boards of numbers stay text
  until `[]G_RID` can draw numbers.

## Waiting on X_eTaL

| Wanted | Meanwhile |
| ------ | --------- |
| enums (statuses, events, moves) | named constants |
| tuples and records (state with fields) | a flat vector and named accessors |
| checked type signatures | `# ::` comments and the type goldens |
| amend (a functional update) | `State`'s update by a table |
| local functions in a lambda | top-level helpers taking the extra values |
| user macros (assertions) | checks printed by the scripted games |
| screen control, single keys | text redrawn each turn, a line per command |
