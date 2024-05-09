```toml
[fuzz]
exec = ["cargo", "run", "--release", "--", "generate"]
env = {}

[judge]
exec = ["cargo", "run", "--release", "--", "validate"]
```

# 🔐 Insecure Password Comparison
A web service stores passwords in their hashed form and compares the hashes when a user wants to log in. However, to improve performance the developers of this service decided to only compare the first 3 characters of the hash accidentally making their service far less secure. Given a password hash and the following hash function, find a password that produces a hash that matches the **first 3 characters** of the one provided. 

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
Your output should be a password that when hashed produces a hash with the **first 3 characters** matching that of the provided hash.
