```toml
[fuzz]
exec = ["cargo", "run", "--release", "--", "generate"]
env = {}

[judge]
exec = ["cargo", "run", "--release", "--", "validate"]
```
# 🎨 Fill Bucket Tool
Given a 100x100 bitmap image, write an algorithm to fill the area of pixels of the same type **starting from position (50, 50)**. Count and **output the number of pixels that needed to be filled**. 

```
  XX        XX
 X  X  ->  XXXX
  X X       XXX
  XX        XX

Output = 3
```


## Input
The input is in a comma separated list with values, `0` for an empty pixel or `1` for a filled pixel. The image will be a crudely drawn circle
```
0, 0, 0, 0, 0
0, 1, 1, 1, 0
1, 0, 0, 0, 1
1, 0, 0, 0, 1
1, 0, 0, 0, 1
0, 1, 1, 1, 0
0, 0, 0, 0, 0
```

## Output
Output the **number of pixels filled**.
