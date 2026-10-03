# Robot chase

Twelve robots chase you across a 16 by 16 board. Each turn you step
(or wait, or teleport), then every robot steps one square toward you.
Robots that land on the same square crash and leave a wreck; a robot
that walks into a wreck is destroyed. Clear the board to win; meet a
robot or a wreck and you lose. It is a port of the COR24 BASIC robot
chase, and the reason it is here: in the BASIC every robot moves in a
`FOR` loop, one at a time; in X_eTaL the robots are one matrix, and
all of them move in one expression, as the game says they do.

Live: [the robot chase page](https://softwarewrighter.github.io/X_eTaL-games/robot-chase/)
-- the game (`play.xtl`) in a terminal, the scripted game
(`robot-chase.xtl`) as a notebook, and the sources: everything on the
page is X_eTaL's own output, run in your browser.

![The robot chase page](screenshot.png)

## The program

`play.xtl` imports the rules as `r:` and loops: show the board, read a
command, `s r:m_ove c`, say what happened.

```
"r:" u_se< "RobotChase"
u:t_urn := { s ->
  shown := p_rint! u:s_how s
  c := u:a_sk s
  c = 10 ? u:t_urn u:s_can s
  t := s r:m_ove c
  0 = r:e_vent t ? u:a_fter t; u:a_fter u:s_ay t
}
```

## The rules: a library

`RobotChase.xtl` (exports named `l:`, seen as `r:`). The whole game
is one flat vector: your row and column, teleports, turn, the last
event, the status, the robots' rows, columns and which are alive, and
the 256 squares of wreckage. The robots' turn:

```
l:c_hase := { s ->
  a := l:a_live s
  W := l:w_recks s
  P := (l:p_os s) 'l_eft t_able r_ange l:n
  d := P - l:r_obots s
  R := (l:r_obots s) + ((2 c_at l:n) r_eshape a) * (d > 0) - d < 0
  k := l:c_ell R
  caught := '| r_/ a * k = l:c_ell l:p_os s
  a1 := a * 1 - k s_elect r_avel W
  crash := a1 * 1 < '+ r_/_2 (k '= t_able k) * (l:n c_at l:n) r_eshape a1
  ...
}
```

## How it works

Read each line right to left.

- `P := (l:p_os s) 'l_eft t_able r_ange l:n`: your position repeated
  into a 2 by 12 matrix, one column per robot, to line up with the
  robots' matrix `R` (row 1 their rows, row 2 their columns).
- `(d > 0) - d < 0` with `d := P - R`: the sign of every difference,
  -1, 0 or 1, for every robot and both coordinates at once. Adding it
  (masked by which robots are alive) moves every robot one step toward
  you: the BASIC's lines 5050 to 5110 in one expression.
- `k := l:c_ell R`: every robot's square number, 1 to 256.
- `(k '= t_able k)`: a 12 by 12 table, 1 where two robots share a
  square; weighted by which are alive and summed along each row, it
  counts the robots on each robot's square, and a count over 1 is a
  crash. The BASIC compares every pair in a nested `FOR I`, `FOR J`.
- New wrecks: a 256 by 12 table of square against robot, kept where a
  robot crashed, `'| r_/_2`-reduced: a 0/1 board of 256 squares, or-ed
  into the wreckage.
- The board you see: three 0/1 boards (robots, wrecks, you) combined
  into one code per square and indexed into `".R*PX "`;
  `2 r_eplicate_2` doubles every column to make room for the spaces.
- The long-range scan: the 256 squares reshaped to a 4 by 4 by 4 by 4
  array (region row, row, region column, column) and summed over axes
  2 and 4 (`'+ r_/_24`): the robots, wrecks and you in each region.
- A new game: 13 different random squares are the first 13 of the
  order (`g_rade`) of 256 random keys.

## Play it

```bash
just play robot-chase        # at the terminal
just run robot-chase         # the scripted game: one step shown, a crash, a loss
just show robot-chase        # the scripted game as a notebook
just repl robot-chase        # then "r:" u_se< "RobotChase" and r:v_iew r:n_ew 0
just test-game robot-chase   # compare with the expected output
```

Commands, as in the BASIC: 7 8 9 move up-left, up, up-right; 4 and 6
sideways; 5 waits; 1 2 3 down; 0 teleports (three a game); 10 scans;
99 resigns.

## Sources

A port of
[`robot-chase.bas`](https://github.com/sw-embed/web-sw-cor24-basic/blob/36569ac624921861122b199a165212710c2f0a2f/examples/robot-chase.bas)
from the COR24 BASIC live demos
([sw-embed/web-sw-cor24-basic](https://github.com/sw-embed/web-sw-cor24-basic),
MIT; the link is to the version ported). The BASIC keeps the board in memory with
`POKE` and `PEEK` and moves robots one at a time:

```
5050 FOR I=0 TO N-1
5060 IF PEEK(340+I)=0 THEN GOTO 5110
5070 LET X=PEEK(300+I)
5071 LET Y=PEEK(320+I)
5080 IF X<A THEN LET X=X+1
5081 IF X>A THEN LET X=X-1
5082 IF Y<B THEN LET Y=Y+1
5083 IF Y>B THEN LET Y=Y-1
```

Differences: the BASIC's generator (`R*97+1 MOD 8191`, seed 5237)
is replaced by `r_oll!`, so boards differ; the scan prints three 4 by
4 count matrices (robots, wrecks, you) side by side where the BASIC
printed `K/M/O` per region; the game's state is one vector, not
memory addresses.

## Assets

None.

## Workarounds

None.
