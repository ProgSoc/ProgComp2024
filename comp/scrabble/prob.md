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

# 🔠 Prefix Scrabble

One day, three inter-galactical linguists, Alice, Bob and Charlie, get extremely bored.
They decide they want to play Scrabble, but with a twist: 'Prefix Scrabble'.
To simplify this version for themselves, this game only uses the letters `a` through to `e` inclusive.

The game of 'Prefix Scrabble' works as follows.

* Each game of 'Prefix Scrabble' starts with our trio
  being given a dictionary of valid words, which will also contain
  the number of points each word grants to the players.
* The three players take turns one after another:
  first Alice, then Bob, then Charlie, and back to Alice, etc.
* Each word formed gives points to all three players at once,
  but will give different points to each person.
  * For example, the word `abc` may give
    7 points to Alice,
    5 points to Bob,
    and 3 points to Charlie when formed.
* Each player takes turns to contribute one **character**
  onto the end of the existing word.
  For example, a game may play out as:
  * Alice starts the game with the letter `e`, forming the word "**e**".
  * Bob then contributes with the letter `c`, forming the word "**ec**".
  * Charlie then contributes with the letter `d`, forming the word "**ecd**".
* A player can *only* contribute a letter that is a prefix of a valid word.
  * For example, if the only valid words are "**ecda**", "**ecdba**", and "**ecdbb**",
    a player can contribute `a` and `b` to add to the existing string "**ecd**",
    but cannot contribute `c`, `d` or `e`.
* The moment a valid word has been formed,
  the game will end and the players will gain the associated points.

Although Alice, Bob and Charlie are very competitive at heart,
they only care about maximising their own points,
even if it means another player gets a higher score than them at the end.

Assuming all participants abide by this strategy,
what is the word they will end up with, and what will be the product of each player's points?

## Example

Consider the following given dictionary.

```
aaa 4 2 1
aba 9 5 4
abb 6 4 5
ac 1 3 2
ba 2 1 7
bba 7 9 6
bbb 8 8 3
bca 3 6 8
bda 5 7 9
```

* If Alice starts the game with `a`,
  Bob will choose to play `b`,
  then Charlie will choose to play `b`.

  The final scores would be: Alice - **6**, Bob - **4**, Charlie - **5**.

* If Alice starts the game with `b`,
  Bob will choose to play `b`,
  then Charlie will choose to play `a`.

  The final scores would be: Alice - **7**, Bob - **9**, Charlie - **6**.

Thus, Alice will choose to play `b`,
with the final word being "**bba**", and the product of their scores being **378**.

## Input

The input will be a dictionary as depicted in the above example.

Each line will consist of four space-separated fields,
the first of which being a valid 'Prefix Scrabble' word.
The next three fields are integers that represent
the number of points this word will grant to Alice, Bob and Charlie respectively.

**NOTE**: Although the example shows the entries sorted in lexicographical order on the word,
this will not be the case in the actual puzzle input.

### Constraints

* Every letter in a valid word is one of `a`, `b`, `c`, `d` and `e`.
* The length of each word is between 1 and 9 inclusive.
* All scores are positive integers.
* No two different words will score the same for one player.

## Output

Your output will be a space-separated string containing the final word formed
followed by the product of Alice, Bob and Charlie's final scores.

For the example above, the output will thus be the following.
```
bba 378
```

The space in the middle is just one regular whitespace character. Nothing fancy.