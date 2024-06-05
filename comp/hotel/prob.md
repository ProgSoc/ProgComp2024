```toml
# TODO: Add clarifications around `<` / `<=` and off-by-one for cleaning times and arrival/departure times

[fuzz]
exec = ["cargo", "run", "--release", "--", "generate"]
env = {}

[judge]
exec = ["cargo", "run", "--release", "--quiet", "--", "validate"]

[problem]
points = 30
difficulty = 3
```
# 🏨 Hotel Rooms
From a list of arrival and departure of hotel guests as timestamps **in seconds**, **allocate a room to each guest** given that each room **must be cleaned before a new guest can stay there**. The hotel's cleaning staff has **5 members** and each room takes **1 member 30 minuets (1800 seconds) to clean**. Each guest must have their own room. The hotel **only has 50 rooms** and your solution must not exceed this.

## Input
Each line of the input represents a guest's arrival time followed by their departure time separated by a comma in Unix seconds.

```
84182, 106880
44259, 68108
49341, 70061
94694, 116988
37514, 51200
83244, 103676
```

## Output
Your output should be a comma-separated list of room numbers **starting from 0** (whitespace is ignored) that corresponds to each line in the input in the order that they are given. The **maximum allowed room number in your answer is 49** as there are only 50 rooms.
```
0, 0, 1, 0, 2, 1, 1, 0
```
