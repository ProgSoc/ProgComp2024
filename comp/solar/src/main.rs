use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    process::exit,
};

use rand::{rngs::StdRng, Rng, SeedableRng};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    assert_eq!(args.len(), 3);

    let mut s = DefaultHasher::new();
    args[2].hash(&mut s);
    let seed = s.finish();

    let mut rng = StdRng::seed_from_u64(seed);

    let alpha = rng.gen_range(0.1..0.5);

    match args[1].as_str() {
        "generate" => {
            println!("{}", alpha);
        }
        "validate" => {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();

            let input = buffer
                .trim()
                .parse::<f64>()
                .expect("Invalid input. Expected number.");

            let solution: f64 = todo!();

            if (input - solution).abs() < 0.01 {
                exit(0);
            } else {
                exit(1);
            }
        }
        _ => panic!(),
    }
}
