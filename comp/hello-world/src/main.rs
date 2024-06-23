use std::{
    collections::{hash_map::DefaultHasher},
    hash::{Hash, Hasher},
    process::exit,
};

use rand::{Rng, SeedableRng};

use rand_chacha::ChaChaRng;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 3);

    // Convert the string into a hash
    let mut default_hasher = DefaultHasher::new();
    args[2].hash(&mut default_hasher);
    let seed = default_hasher.finish();

    let mut rng = ChaChaRng::seed_from_u64(seed);

    let name_length = rng.gen_range(3..=10);

    let first_char = char::from_u32(rng.gen_range(65..=90)).unwrap();

    let string_tail: String = (0..name_length)
        .map(|_| {
            char::from_u32(rng.gen_range(97..=122)).unwrap()
        }).collect();

    let name = format!("{first_char}{string_tail}");

    match args[1].as_str() {
        "generate" => {
            println!("{name}");
        }
        "validate" => {
            let solution_string = format!("Hello {name}!");

            let mut buffer = String::new();
            let _ = std::io::stdin().read_line(&mut buffer);

            if buffer.trim() == solution_string {
                exit(0);
            } else {
                exit(1);
            }
        }
        _ => panic!(),
    }
}