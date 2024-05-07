use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    process::exit,
};

use primes::is_prime;
use rand::{rngs::StdRng, Rng, SeedableRng};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    assert_eq!(args.len(), 3);

    let mut s = DefaultHasher::new();
    args[2].hash(&mut s);
    let seed = s.finish();

    let mut rng = StdRng::seed_from_u64(seed);

    let n = rng.gen_range(700..10_000);

    match args[1].as_str() {
        "generate" => {
            println!("{}", n);
        }
        "validate" => {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();

            let input = buffer.trim();

            is_valid_tx_chars(input.to_string()).unwrap();

            let input = parse_tx(input.to_string()).unwrap();

            if input == n {
                exit(0);
            } else {
                exit(1);
            }
        }
        _ => panic!(),
    }
}

fn is_valid_tx_chars(s: String) -> Result<(), String> {
    for c in s.chars() {
        if c != ':' && c != '(' && c != ')' && c != '-' && c != 'P' {
            return Err("Invalid character(s). If this was meant to be numerical it failed to parse. Make sure it fits in a 64-bit unsigned integer.".to_string());
        }
    }
    Ok(())
}

fn parse_tx(tx: String) -> Result<u64, String> {
    let mut child_call_buffer = "".to_string();
    let mut n = 1;
    let mut p: i64 = 0;
    let mut brack_depth = 0;
    for c in tx.chars() {
        if c == ')' {
            brack_depth -= 1;
            if brack_depth == 0 {
                run_recusivly_and_flush_buffer(&mut child_call_buffer, &mut p, &mut n)?;
            }
        }
        if brack_depth > 0 {
            child_call_buffer.push(c);
        } else if c == ':' {
            n <<= 1;
        } else if c == 'P' || c == '-' {
            return Err(format! {"Found unexpected \"{}\".", c});
        }
        if c == '(' {
            brack_depth += 1
        }
    }
    if brack_depth != 0 {
        return Err("Unbalanced brackets.".to_string());
    }
    let r = n as i128 + p as i128;
    Ok(r as u64)
}

fn run_recusivly_and_flush_buffer(
    child_call_buffer: &mut String,
    p: &mut i64,
    n: &mut u64,
) -> Result<(), String> {
    Ok(if *child_call_buffer == "-P" {
        *p -= 1;
    } else if *child_call_buffer == "(-P)" {
        *p -= 2
    } else {
        let child = parse_tx(child_call_buffer.clone())?;
        *child_call_buffer = "".to_string();
        *n *= nth_prime(child);
    })
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
