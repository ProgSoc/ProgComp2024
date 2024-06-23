```toml
[fuzz]
exec = ["cargo", "run", "--release", "--", "generate"]
env = {}

[judge]
exec = ["cargo", "run", "--release", "--quiet", "--", "validate"]

[problem]
points = 9
difficulty = 1
```

# 📜 Bytecode Interpreter

"To know a computer, one must become a computer."
At least, that's what your manager said when they put this task on your desk.

Anyway. Time to read.
Apparently, someone has left behind a large list of instructions, in a certain kind of bytecode.

The interpreter reads this code line by line and performs each instruction's action as it goes.

There are 4 kinds of instructions encoded in this bytecode.
* **`LABEL <int>`** - This is a marker in the code that can be jumped to. It does not perform any actions.
  * For example, `LABEL 20` can be jumped to by a command like `JZ a 20`.
* **`ADD <var> <int>`** - Increase the value of variable `<var>` by `<int>`.
  This can be any integer (positive or negative).
  * For example, `ADD a 123` will add `123` to the existing value of the variable `a`,
    and put this value back into the variable `a`.
* **`JZ <var> <int>`** - If the value of variable `<var>` is `0`, jump to `LABEL <int>`.
  If it jumps to a non-existent label, the program ends.
  * For example, `JZ a 7` will jump to `LABEL 7` if the variable `a` currently contains the value of zero.
* **`COPY <var1> <var2>`** - Puts the value of `<var1>` into the variable `<var2>`.
  * For example, if `COPY a b` is executed when `a` has the value of `50`,
    then `b` will now also have the value `50`.

At the start of the program,
there are 5 integer variables `a`, `b`, `c`, `d`, and `e`
that all initially have the value of `0`.

Right as your manager leaves,
a passing comment is made that the code you are given may pontentially contain an infinite loop,
and in such cases, you reckon that executing 5000 instructions should be enough.

Therefore, we seek to answer the question:
**Given a list of bytecode instructions,
after the program finishes,
or after 5000 instructions have been executed (whichever happens first),
what is the value contained in the variable `a`?**

## Example

Consider this list of instructions.

```
ADD a -1
LABEL 0
ADD a 1
JZ a 0
COPY a b
```

In the above example, the following instructions will be executed.

* **`ADD a -1`** - `a` is given the value of `0 + (-1) = -1`.
  (Remember, `a` is initialised with `0`.)
* **`LABEL 0`** - No action is taken, but any `JZ <var> 0` instruction may redirect here.
* **`ADD a 1`** - We add `1` to `a`, so `a` now has the value of `-1 + 1 = 0`.
* **`JZ a 0`** - We check the value of `a` against zero.
  Since `a` is currently `0`, we will now jump to `LABEL 0`.
* **`LABEL 0`** - No action is taken.
* **`ADD a 1`** - We add `1` to `a`, so `a` now has the value of `0 + 1 = 1`.
* **`JZ a 0`** - We check the value of `a` against zero.
  Since `a` is currently `1`, we do **not** jump to `LABEL 0`.
  Instead, we move to the next instruction.
* **`COPY a b`** - We copy the value of `a` (which is currently `1`) into variable `b`.
  `b` now has the value of `1`.

**At the end of executing these instructions, `a` has the value of `1`,
and we have executed 8 instructions.**

## Input
Your input will be a list of bytecode instructions.
Each instruction will be on a new line, and
the operands of the instruction will be separated by spaces, as per the example above.

### Constraints

* All variables are within the signed 64-bit integer limit.
* All `LABEL` numbers are non-negative integers.
* There are only five variables: `a`, `b`, `c`, `d`, and `e`.

## Output
Your output should be a single integer: the value of variable `a` after the program has finished executing
(or 5000 instructions have been executed, whichever happens first).

For the example above, the output will be the following.
```
1
```