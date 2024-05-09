fn hash(s: String) -> String {
    const MAGIC: i128 = 123123;

    let mut h: i128 = 0; 

    for c in s.chars() {
        h += c as i128 * MAGIC;
        h ^= MAGIC;
        h <<= 2;
        h %= 1 << 32;
    }

    return h.to_string();
}
