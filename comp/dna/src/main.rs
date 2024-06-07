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

    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    const LENGTH: usize = 100;

    let seq1 = new_sequence(LENGTH, &mut rng);
    let seq2 = new_sequence(LENGTH, &mut rng);

    match args[1].as_str() {
        "generate" => {
            println!("{}", seq1);
            println!("{}", seq2);
        }
        "validate" => {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();

            let similarity = similarity(&seq1, &seq2);

            let input_similarity = buffer
                .trim()
                .parse::<f64>()
                .map_err(|_| ())
                .and_then(|x| if x >= 0.0 && x <= 1.0 { Ok(x) } else { Err(()) })
                .graceful_expect("Invalid input. Expected a number beteen 0 and 1.");

            if (similarity - input_similarity).abs() < 0.0001 {
                exit(0);
            } else {
                exit(1);
            }
        }
        _ => panic!(),
    }
}

trait GracefulExpect<T> {
    fn graceful_expect(self, message: &str) -> T;
}

impl<T, E> GracefulExpect<T> for Result<T, E> {
    fn graceful_expect(self, message: &str) -> T {
        match self {
            Ok(v) => v,
            Err(_) => {
                eprintln!("{}", message);
                exit(1);
            }
        }
    }
}

static NEUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

fn new_sequence(length: usize, rng: &mut StdRng) -> String {
    let mut sequence = String::new();
    for _ in 0..length {
        let index = rng.gen::<usize>() % NEUCLEOTIDES.len();
        sequence.push(NEUCLEOTIDES[index]);
    }
    sequence
}

fn similarity(seq1: &str, seq2: &str) -> f64 {
    let mut matches = 0;
    for (n1, n2) in seq1.chars().zip(seq2.chars()) {
        if n1 == n2 {
            matches += 1;
        }
    }
    matches as f64 / seq1.len() as f64
}
