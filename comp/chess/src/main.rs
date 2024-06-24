use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    process::exit,
};

use rand::{seq::SliceRandom, SeedableRng};

use rand_chacha::ChaChaRng;

use crate::magics::*;
use crate::movegen::*;

mod bitboard;
mod magics;
mod movegen;

fn init() -> (Vec<SMagic>, Vec<SMagic>) {
    (generate_bishop_table(), generate_rook_table())
}

#[allow(dead_code)]
fn square_to_string(sq: u64) -> String {
    let f = ["a", "b", "c", "d", "e", "f", "g", "h"];
    let r = ["1", "2", "3", "4", "5", "6", "7", "8"];
    format!("{}{}", f[(sq & 7) as usize], r[((sq >> 3) & 7) as usize])
}

#[allow(dead_code)]
fn move_string(mv: u64) -> String {
    let o = mv & 0x3f;
    let d = (mv >> 6) & 0x3f;
    format!(
        "{}{}",
        square_to_string(o),
        square_to_string(d)
    )
}

fn board_string(pos: Position) -> Vec<String> {
    let mut grid = ['.'; 64];
    for i in 0..64 {
        if ((pos.own >> i) & 1) == 1 {
            if (((pos.ortho & pos.diag) >> i) & 1) == 1 {
                grid[i] = 'Q';
            } else if ((pos.ortho >> i) & 1) == 1 {
                grid[i] = 'R';
            } else if ((pos.diag >> i) & 1) == 1 {
                grid[i] = 'B';
            } else if pos.kings & 0x3f == i as u64 {
                grid[i] = 'K';
            } else {
                grid[i] = 'N';
            }
        } else if ((pos.other >> i) & 1) == 1 {
            if (((pos.ortho & pos.diag) >> i) & 1) == 1 {
                grid[i] = 'q';
            } else if ((pos.ortho >> i) & 1) == 1 {
                grid[i] = 'r';
            } else if ((pos.diag >> i) & 1) == 1 {
                grid[i] = 'b';
            } else if (pos.kings >> 6) & 0x3f == i as u64 {
                grid[i] = 'k';
            } else {
                grid[i] = 'n';
            }
        }
    }
    (0..64)
        .step_by(8)
        .rev()
        .map(|i| grid[i..i + 8].iter().collect::<String>())
        .collect()
}

fn main() {
    let (bishop_magics, rook_magics) = init();

    let args: Vec<String> = std::env::args().collect();

    assert_eq!(args.len(), 3);

    // Convert the string seed into a hash
    let mut default_hasher = DefaultHasher::new();
    args[2].hash(&mut default_hasher);
    let seed = default_hasher.finish();

    let mut rng = ChaChaRng::seed_from_u64(seed);

    // Generate random position.
    let mut position = Position {
        ortho: 0,
        diag: 0,
        own: 0,
        other: 0,
        kings: 0,
    };

    let sample: Vec<usize> = (0..64)
        .collect::<Vec<usize>>()
        .choose_multiple(&mut rng, 22)
        .map(|&x| x)
        .collect();

    for (i, square) in sample.iter().map(|&x| x as u64).enumerate() {
        match i {
            0..=13 => {
                position.own |= 1 << square;
                match i {
                    0 => {
                        position.kings |= square;
                    }
                    1..=2 => {
                        position.ortho |= 1 << square;
                        position.diag |= 1 << square;
                    }
                    3..=5 => {
                        position.ortho |= 1 << square;
                    }
                    6..=9 => {
                        position.diag |= 1 << square;
                    }
                    _ => {}
                }
            }
            14..=21 => {
                position.other |= 1 << square;
                match i {
                    14 => {
                        position.kings |= square << 6;
                    }
                    15 => {
                        position.ortho |= 1 << square;
                        position.diag |= 1 << square;
                    }
                    16..=17 => {
                        position.ortho |= 1 << square;
                    }
                    18..=19 => {
                        position.diag |= 1 << square;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    match args[1].as_str() {
        "generate" => {
            for line in board_string(position) {
                println!("{line}");
            }
        }
        "validate" => {
            // First find the solution ourselves.
            let final_value = perft(position, 4, &bishop_magics, &rook_magics);
            let solution_string = final_value.to_string();

            // Then we get the input.
            let mut buffer = String::new();
            let _ = std::io::stdin().read_line(&mut buffer);

            if buffer.trim() == solution_string {
                exit(0);
            } else {
                if let Ok(value) = buffer.trim().parse::<i64>() {
                    if value < final_value as i64 {
                        eprintln!("Your answer was too low.");
                    } else if value > final_value as i64 {
                        eprintln!("Your answer was too high.");
                    }
                } else {
                    eprintln!("Expected 64-bit integer.");
                }
                exit(1);
            }
        }
        _ => panic!(),
    }
}
