```toml
[fuzz]
exec = ["cargo", "run", "--release", "--", "generate"]
env = {}

[judge]
exec = ["cargo", "run", "--release", "--quiet", "--", "validate"]

[problem]
points = 27
difficulty = 3
```

# 🛸 Esoteric Number Format
Given an integer >= 2, convert it to the following format: Numbers are represented by their factors. 2 is represented by `:`, so 4 is `::`, 8 is `:::`, 16 is `::::` and so on. Some number n in a pair of parentheses represents the nth prime number so `(:)` is the 2nd prime number which is 3, `((:))` is the 3rd prime number which is 5.

* 2 -> `:`
* 3 -> `(:)`
* 4 -> `::`
* 5 -> `((:))`
* 6 -> `:(:)`
* 7 -> `(::)`
* 8 -> `:::`
* 9 -> `(:)(:)`
* 10 -> `:((:))`
* 100 -> `::((:))((:))`
* 23423142134 -> `:(::)(::::)(::((((:)))))((::)(::((((:))))))`

## Input
The input is an integer >= 2.

## Output
The output should be a string of the input number in the format specified above (e.g. `:(:)(:::)((:)(:))((:)((:)))`).
