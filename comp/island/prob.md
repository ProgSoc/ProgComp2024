```toml
[fuzz]
exec = ["cargo", "run", "--release", "--", "generate"]
env = {}

[judge]
exec = ["cargo", "run", "--release", "--quiet", "--", "validate"]

[problem]
points = 15
difficulty = 2
```

# 🏝️ Recursive Islands
Given a 2D array of elevations, locate all [recursive islands](https://en.wikipedia.org/wiki/Recursive_islands_and_lakes) (**islands within lakes within islands**) and return their collective area. **Each entry in the array represents 1 square meter**. Any elevation values greater than `0.0` are land and any elevations less than or equal to `0.0` are water. Bodies of water are not considered connected if they are joined by diagonal points. Assume that everything outside of the provided area is ocean. If a body of water is connected to the ocean it is not considered a lake.


## Input
Your input is set of comma-separated elevation values where each line is a row and the commas separate elevations from each column.
```
-0.2, -0.2, -0.2, -0.2, -0.2 
-0.2, -0.1, 0.1, 0.1, 0.0 
-0.2, -0.1, 0.2, 0.2, 0.0 
-0.2, -0.1, 0.3, 0.2, -0.1 
-0.2, -0.1, 0.2, 0.2, -0.2 
-0.2, -0.1, -0.1, -0.2, -0.2 
```

## Output
Your output should be the collective area of all recursive islands.

