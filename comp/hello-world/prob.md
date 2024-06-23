```toml
[fuzz]
exec = ["cargo", "run", "--release", "--", "generate"]
env = {}

[judge]
exec = ["cargo", "run", "--release", "--quiet", "--", "validate"]

[problem]
points = 1
difficulty = 1
```

# 👋 Hello Programmers!

Say hello to your fellow programmers!

In this problem we'll be greeting people.

For example, `Linus` will be converted to `Hello Linus!`.

## Input format

The first line contains a string `S` which is the name to be greeted.

S is at least one character long, and contains only letters.

## Output format

Output the greeting to the name.