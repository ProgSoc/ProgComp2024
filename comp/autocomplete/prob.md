```toml
[fuzz]
exec = ["cargo", "run", "--release", "--", "generate"]
env = {}

[judge]
exec = ["cargo", "run", "--release", "--quiet", "--", "validate"]

[problem]
points = 13
difficulty = 2
```

# 💬 Auto-Complete Permutations
Given a section of text specifying all of the allowed words and grammatical rules, determine the **number of possible unique 5-word sentences**.

A section of text includes all of the allowed words and what words are allowed to come after that word. For example:
```hello this is some text this is only a test```
* The words that are allowed to come after `is` are `some` and `only`.
* The only word that is allowed to come after `hello` is `this`.
* There are no words that are allowed to come after `test`.

For the sentences you generate:
* They may start with any word in the text.
* They must be exactly 5 words long.
* They may include the same word multiple times.

## Input
Your input will be a section of text with lower-case words separated by spaces with no punctuation.

## Output
Your output should be the number of possible unique 5-word sentences that can be generated from the section of text.
