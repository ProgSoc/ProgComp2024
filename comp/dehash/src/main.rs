use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}, process::exit};

use rand::{rngs::StdRng, Rng, SeedableRng};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    assert_eq!(args.len(), 3);

    let mut s = DefaultHasher::new();
    args[2].hash(&mut s);
    let seed = s.finish();

    let our_hash = hash(random_string(seed));

    match args[1].as_str() {
        "generate" => {
            println!("{}", our_hash);
        }
        "validate" => {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();

            let user_hash = hash(buffer.trim().to_string());

            if compare_head(our_hash, user_hash) {
                exit(0);
            } else {
                exit(1);
            }
        }
        _ => panic!(),
    }
}

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

static LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.";

fn random_string(seed: u64) -> String {
    let mut rng = StdRng::seed_from_u64(seed);

    let length = rng.gen_range(40..60);
    let start = rng.gen_range(0..LOREM_IPSUM.len() - length);

    return LOREM_IPSUM[start..start + length].to_string();
}

fn compare_head(a: String, b: String) -> bool {
    const LENGTH: usize = 3;

    if a.len() < LENGTH || b.len() < LENGTH {
        return false;
    }

    return a.chars().take(LENGTH).eq(b.chars().take(LENGTH));
}

