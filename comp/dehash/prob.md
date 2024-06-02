```toml
[fuzz]
exec = ["cargo", "run", "--release", "--", "generate"]
env = {}

[judge]
exec = ["cargo", "run", "--release", "--", "validate"]

[problem]
points = 5
difficulty = 1
```

# 🔐 Insecure Password Comparison
A web service stores passwords in their hashed form and [compares the hashes when a user wants to log in](https://en.wikipedia.org/wiki/Cryptographic_hash_function#Password_verification). However, to improve performance the developers of this service decided to only compare the first 3 characters of the hash accidentally making their service far less secure. Given a password hash and the following hash function, find an ASCII password that produces a hash with the **first 3 characters** matching those in the hash provided.

```python
def hash(s):
    magic = 123123

    h = 0
    for c in s:
        h += ord(c) * magic
        h ^= magic
        h <<= 2 
        h %= 1 << 32;

    return str(h)
```

> ***Hint:*** Hash functions for passwords are designed to be *one-way encryption* and thus trying to reverse the function will not be effective.

## Input
Your input is the hash of the original password.

## Output
Your output should be an ASCII password that when hashed produces a hash with the **first 3 characters** matching those of the provided hash.
