```toml
[fuzz]
exec = ["cargo", "run", "--release", "--", "generate"]
env = {}

[judge]
exec = ["cargo", "run", "--release", "--quiet", "--", "validate"]

[problem]
points = 20
difficulty = 3
```


# ♟️ Counting Chess Positions

After deciding to not play further FIDE World Chess Championships,
Magnus Carlsen got bored of regular old chess to the point where
he has resorted to counting hypothetical chess positions.
He's decided to take a page out of the low-ELO book,
inspired by their obssession with the Scholar's mate pattern.
Thus, Magnus now wants to answer the following question.

Given a certain chess position (from White's perspective),
if White could make 4 **pseudo-legal** moves in a row,
how many unique move sequences would be possible?

For simplicity, all positions will not contain any pawns.

A **pseudo-legal** move for White in this context refers to a move that has the following properties:

* Moves a piece (within their movement restrictions) to either
  an empty square or a square occupied by a Black piece,
  **with the exception of the Black king**.
* May potentially check the Black king and/or leave the White king in check when the move is made.

Additionally, we are counting **unique move sequences**,
so if we ever reach a previously reached *position* from a different *move order*,
we still count them as different.

## Notation

To represent a board state, the occupancy of each square is represented by a singular character, as follows.
(Keep in mind capitalisation matters.)

* `K`: White king
* `Q`: White queen
* `R`: White rook
* `B`: White bishop
* `N`: White knight
* `k`: Black king
* `q`: Black queen
* `r`: Black rook
* `b`: Black bishop
* `n`: Black knight
* `.`: Empty space

## Example

Consider the following board, annotated with ranks (`1` to `8`) and files (`a` to `h`).

```
8  .....N..
7  ..K.R..b
6  ..B...k.
5  .N.....n
4  .....r..
3  ...r.n..
2  .b..Q.B.
1  .Rq.....

   abcdefgh
```

Consider the White knight on **f8**.

* Moving to the empty square **d7** is a valid pseudo-legal move,
* Moving to the empty square **e6** is a valid pseudo-legal move,
* Capturing the Black bishop on **h7** is a valid pseudo-legal move,
* Capturing the Black king on **g6** is **not** a valid pseudo-legal move.

Now consider the White rook on **e7**.

* Moving to the empty square **d7** is a valid pseudo-legal move, as well as the other empty squares
  **f7**, **g7**, **e3**, **e4**, **e5**, **e6**, and **e8**.
* Capturing the Black bishop on **h7** is a valid pseudo-legal move,
* Moving to the occupied squares **c7** (by the White king) and **e2** (by the White queen)
  are **not** valid pseudo-legal moves.

By considering all the White pieces on the board in this way,
there are 51 pseudo-legal moves in the above position.

For this example, if White could make 4 pseudo-legal moves in a row,
we would ultimately end up with **7320026** possible unique sequences of moves.

## Input

The input will be a grid of 8 characters by 8 characters,
like in the example above, but without the rank and file annotations.
That is, for the same depicted position, the puzzle input would be as follows.
```
.....N..
..K.R..b
..B...k.
.N.....n
.....r..
...r.n..
.b..Q.B.
.Rq.....
```

### Constraints

* All characters in the grid will be one of the characters listed in the Notation section (with the exception of newlines).
* All grids will be 8x8.
* There will be exactly 1 White king and 1 Black king in all positions.

## Output

A single number representing the number of unique move sequences.

For the example above, the output will be the following.
```
7320026
```