```toml
[fuzz]
exec = ["cargo", "run", "--release", "--", "generate"]
env = {}

[judge]
exec = ["cargo", "run", "--release", "--quiet", "--", "validate"]

[problem]
points = 14
difficulty = 2
```

# 🏤 Two Paths

Two travelling salesmen, Mario and Luigi, come across a region full of cities and one-way bridges
(that is, bridges that have one-way traffic) connecting them.
Crossing a bridge will grant the salesman a reward of `x` dollars.

They intend to cross bridges this way,
going from one city to the next, such that they walk a path that
starts from City `aaa` and ends in City `zzz`.
Additionally, Mario will always take the path that gives him the **most** money,
and Luigi (for some reason) will always take the path that gives him the **least** money.

Princess Peach has said that after both Mario and Luigi conclude their commutes,
she will pay them the amount of money equal to the **product** of Mario's collected money
and Luigi's collected money. For example, if Mario collects **$15** and Luigi collects **$5**,
Princess Peach will pay out **$75**.

Help Mario and Luigi answer the following question:
Given a certain map, how much money would Princess Peach give them?

## Input

The input will be formatted such that each line represents one city in the region,
as well as each of the *outgoing* bridges from that city and its destination and associated reward.

Every city that exists in the region is guaranteed to have a line of its own in the input.

### Example

Consider the following input.
```
aaa -> bbb: 1, ccc: 3
bbb -> ccc: 1, ddd: 6
ccc -> bbb: 1, ddd: 2, eee: 4
ddd -> bbb: 6, ccc: 2, eee: 4, zzz: 5
eee -> ccc: 4, ddd: 4, zzz: 2
zzz -> ddd: 5, eee: 2
```

There are 6 cities (including the starting and ending cities `aaa` and `zzz`).

The first line of this input indicates that there are two outgoing bridges from City `aaa`:

* A bridge to City `bbb` yielding a reward of **$1**,
* A bridge to City `ccc` yielding a reward of **$3**.

Following this logic, the lines above describe the topology of the entire region to be traversed.

In this case,
* The path Mario will follow is `aaa -> ccc -> eee -> ddd -> zzz`,
  amassing a reward of $3 + $4 + $4 + $5 = **$16**.
* The path Luigi will follow is `aaa -> bbb -> ccc -> eee -> zzz`,
  amassing a reward of $1 + $1 + $4 + $2 = **$8**.

Therefore, in this example, Princess Peach will give them **$128** in reward.

### Constraints

* All city names are three characters in length, consisting only of lowercase English characters.
* Both Mario and Luigi must start from City `aaa` and end in City `zzz`.
* All rewards associated with bridges are positive integers.
* Paths must not pass through the same city twice. For example,
  * `aaa -> bbb -> zzz` and `aaa -> zzz` are valid paths,
  * `aaa -> bbb -> ccc -> bbb -> zzz` and `aaa -> aaa -> zzz` are invalid paths.

## Output

A single number representing the amount of money Princess Peach will give to Mario and Luigi.

For the example above, the output will be the following.
```
128
```