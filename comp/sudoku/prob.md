```toml
[fuzz]
exec = ["cargo", "run", "--release", "--", "generate"]
env = {}

[judge]
exec = ["cargo", "run", "--release", "--", "validate"]
```

# 🔢 Sudoku Solutions
Given a sudoku puzzle, determine the **number of unique solutions**.

## Rules of Sudoku
A valid solution to a sudoku puzzle has one of each number from 1 to 9 in every column, row and 3x3 segment. Rows, columns and segments cannot contain a number more than once. The 3x3 segments are are segments in the thicker lines in the example game below. A puzzle starts with several given values and the player must fill in the remaining squares.

```
┏━━┯━━┯━━┳━━┯━━┯━━┳━━┯━━┯━━┓ 
┃4 │3 │9 ┃5 │7 │1 ┃8 │2 │6 ┃
┠──┼──┼──╂──┼──┼──╂──┼──┼──┨
┃7 │1 │8 ┃  │  │  ┃  │  │  ┃
┠──┼──┼──╂──┼──┼──╂──┼──┼──┨
┃5 │6 │2 ┃  │  │  ┃  │  │  ┃
┣━━┿━━┿━━╋━━┿━━┿━━╋━━┿━━┿━━┫
┃  │  │1 ┃  │  │7 ┃  │  │  ┃
┠──┼──┼──╂──┼──┼──╂──┼──┼──┨
┃  │  │4 ┃  │  │  ┃  │  │  ┃
┠──┼──┼──╂──┼──┼──╂──┼──┼──┨
┃9 │  │7 ┃  │  │  ┃  │  │  ┃
┣━━┿━━┿━━╋━━┿━━┿━━╋━━┿━━┿━━┫
┃  │  │3 ┃  │  │  ┃  │  │  ┃
┠──┼──┼──╂──┼──┼──╂──┼──┼──┨
┃  │  │5 ┃  │4 │  ┃  │3 │  ┃
┠──┼──┼──╂──┼──┼──╂──┼──┼──┨
┃  │  │6 ┃  │  │  ┃  │  │  ┃
┗━━┷━━┷━━┻━━┷━━┷━━┻━━┷━━┷━━┛

```

## Input
The input is a 9x9 array of comma separated values. **0 represents an empty square** and 1-9 are filled in squares.
```
0, 2, 0, 0, 0, 0, 0, 0, 0
5, 0, 0, 1, 0, 9, 0, 7, 0
4, 7, 9, 0, 0, 0, 0, 2, 0
2, 0, 1, 5, 4, 0, 0, 0, 0
6, 8, 0, 7, 9, 2, 0, 0, 0
7, 9, 0, 0, 0, 3, 0, 6, 0
0, 1, 0, 0, 0, 4, 9, 5, 0
0, 0, 0, 2, 3, 0, 6, 0, 8
0, 0, 0, 9, 7, 5, 0, 3, 1
```

## Output
Output the number of unique valid solutions to the puzzle.
