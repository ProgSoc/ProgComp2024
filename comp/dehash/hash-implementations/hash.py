def hash(s):
    magic = 123123

    h = 0
    for c in s:
        h += ord(c) * magic
        h ^= magic
        h <<= 2
        h %= 1 << 32

    return str(h)
