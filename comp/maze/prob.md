```toml
[fuzz]
exec = ["cargo", "run", "--release", "--", "generate"]
env = {}

[judge]
exec = ["cargo", "run", "--release", "--quiet", "--", "validate"]

[problem]
points = 12
difficulty = 2
```

# 🧭 Maze Solver
Given the layout of a maze with **one correct route**, find the length of said route. The maze is represented by a graph where nodes are certain points in the maze and edges are paths between them. You are looking for the **number of nodes in between the start (ID = 0) and end (ID = 1)**.

## Input
Each line specifies a position in the maze. Number at the head of each line is that line's unique ID. Following the colon is a comma-separated list of the IDs of all other positions in the maze with a path from the position represented by that line. **The first item in the list (with ID = `0`) is the start and the second (with ID = `1`) is the end**.

```
0: 2
1: 6
6: 1, 5
4: 5, 2
2: 0, 3
3: 2, 4
5: 6, 4
```
In the example above, there is a path from point `0` to point `2`, from point `2` back to point `0` and to point `3`, ect.

## Output
The output is the number of nodes in between the start (ID = `0`) and the end (ID = `1`). There is only one route between them so this solution is unique.

