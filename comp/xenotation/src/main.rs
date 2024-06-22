use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    process::exit,
};

use primes::is_prime;
use rand::{Rng, SeedableRng};

use rand_chacha::ChaChaRng;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    assert_eq!(args.len(), 3);

    let mut s = DefaultHasher::new();
    args[2].hash(&mut s);
    let seed = s.finish();

    let mut rng = ChaChaRng::seed_from_u64(seed);

    let n = rng.gen_range(700..10_000);

    match args[1].as_str() {
        "generate" => {
            println!("{}", n);
        }
        "validate" => {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();

            let input = buffer.trim();

            is_valid_tx_chars(input.to_string()).graceful_unwrap();

            let input = parse_tx(input.to_string()).graceful_unwrap();

            if input == n {
                exit(0);
            } else {
                exit(1);
            }
        }
        _ => panic!(),
    }
}

trait GracefulUnwrap<T> {
    fn graceful_unwrap(self) -> T;
}

impl<T> GracefulUnwrap<T> for Result<T, String> {
    fn graceful_unwrap(self) -> T {
        match self {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        }
    }
}

fn is_valid_tx_chars(s: String) -> Result<(), String> {
    for c in s.chars() {
        if c != ':' && c != '(' && c != ')' {
            return Err("Invalid character(s).".to_string());
        }
    }
    Ok(())
}

fn parse_tx(tx: String) -> Result<u64, String> {
    let mut child_call_buffer = "".to_string();
    let mut n = 1;
    let mut brack_depth = 0;

    for c in tx.chars() {
        if c == ')' {
            brack_depth -= 1;
            if brack_depth == 0 {
                run_recusivly_and_flush_buffer(&mut child_call_buffer, &mut n)?;
            }
        }

        if brack_depth > 0 {
            child_call_buffer.push(c);
        } else if c == ':' {
            n <<= 1;
        }

        if c == '(' {
            brack_depth += 1
        }
    }
    if brack_depth != 0 {
        return Err("Unbalanced brackets.".to_string());
    }
    Ok(n as u64)
}

fn run_recusivly_and_flush_buffer(
    child_call_buffer: &mut String,
    n: &mut u64,
) -> Result<(), String> {
    let child = parse_tx(child_call_buffer.clone())?;
    *child_call_buffer = "".to_string();
    *n *= nth_prime(child);
    Ok(())
}

fn nth_prime(n: u64) -> u64 {
    let mut i = 0;
    let mut p = 0;
    while i < n {
        p = next_prime(p);
        i += 1;
    }
    p
}

fn next_prime(n: u64) -> u64 {
    let mut p = n + 1;
    while !is_prime(p) {
        p += 1;
    }
    p
}
