use std::{
    collections::{hash_map::DefaultHasher, BTreeMap},
    hash::{Hash, Hasher},
    process::exit,
};

use rand::{Rng, SeedableRng};

use rand_chacha::ChaChaRng;

#[derive(Clone, Copy, Debug)]
enum ByteCode {
    Label(usize),
    Add(char, i64),
    Jz(char, usize),
    Copy(char, char),
}

impl ByteCode {
    fn to_string(&self) -> String {
        match *self {
            Self::Label(idx) => format!("LABEL {idx}"),
            Self::Add(var, val) => format!("ADD {var} {val}"),
            Self::Jz(var, idx) => format!("JZ {var} {idx}"),
            Self::Copy(src, dst) => format!("COPY {src} {dst}"),
        }
    }
}

fn random_bytecode<R: Rng>(rng: &mut R, next_label: usize) -> ByteCode {
    let vars = ['a', 'b', 'c', 'd', 'e'];

    if rng.gen_bool(0.2) {
        ByteCode::Label(next_label)
    } else if rng.gen_bool(0.5) {
        let var = if rng.gen_bool(0.5) { 'a' } else { vars[rng.gen_range(0..=4)] };
        ByteCode::Add(var, rng.gen_range(1..=5000) as i64)
    } else if rng.gen_bool(0.2) {
        let var = if rng.gen_bool(0.5) { 'a' } else { vars[rng.gen_range(0..=4)] };
        ByteCode::Jz(var, rng.gen_range(1..=next_label))
    } else {
        let var = if rng.gen_bool(0.5) { 'a' } else { vars[rng.gen_range(0..=4)] };
        ByteCode::Copy(var, vars[rng.gen_range(0..=4)])
    }
}

fn random_instructions<R: Rng>(rng: &mut R, size: usize) -> Vec<ByteCode> {
    let mut next_label: usize = 1;
    let mut instructions: Vec<ByteCode> = Vec::new();
    for _ in 0..size {
        let bytecode = random_bytecode(rng, next_label);
        match bytecode {
            ByteCode::Label(label) => {
                next_label = label + 1;
            }
            _ => {}
        }
        instructions.push(bytecode);
    }
    instructions
}

fn execute(cmds: &Vec<ByteCode>, iterations: u64) -> i64 {
    let all_labels: BTreeMap<usize, usize> = cmds
        .iter()
        .enumerate()
        .filter_map(|(idx, &byte_code)| {
            match byte_code {
                ByteCode::Label(label_num) => Some((label_num, idx)),
                _ => None,
            }
        })
        .collect();

    let mut variables: BTreeMap<char, i64> = BTreeMap::from([
        ('a', 0),
        ('b', 0),
        ('c', 0),
        ('d', 0),
        ('e', 0),
    ]);

    let mut i: u64 = 0;
    let mut pc: usize = 0;
    let length = cmds.len();

    while pc < length && i < iterations {
        match cmds[pc] {
            ByteCode::Label(_) => {
                pc += 1;
            }
            ByteCode::Add(var, val) => {
                variables.insert(var, *variables.get(&var).unwrap() + val);
                pc += 1;
            }
            ByteCode::Jz(var, idx) => {
                if let Some(label) = all_labels.get(&idx) {
                    pc = if *variables.get(&var).unwrap() == 0 {
                        *label
                    } else {
                        pc + 1
                    };
                } else {
                    break;
                }
            }
            ByteCode::Copy(src, dst) => {
                variables.insert(dst, *variables.get(&src).unwrap());
                pc += 1;
            }
        }
        i += 1;
    }

    *variables.get(&'a').unwrap()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    assert_eq!(args.len(), 3);

    // Convert the string into a hash
    let mut default_hasher = DefaultHasher::new();
    args[2].hash(&mut default_hasher);
    let seed = default_hasher.finish();

    let mut rng = ChaChaRng::seed_from_u64(seed);

    // Generate random instruction list.
    let instructions = random_instructions(&mut rng, 1000);

    match args[1].as_str() {
        "generate" => {
            for cmd in instructions {
                println!("{}", cmd.to_string());
            }
        }
        "validate" => {
            // First find the solution ourselves.
            let final_value = execute(&instructions, 5000);
            let solution_string = final_value.to_string();

            // Then we get the input.
            let mut buffer = String::new();
            let _ = std::io::stdin().read_line(&mut buffer);

            if buffer.trim() == solution_string {
                exit(0);
            } else {
                if let Ok(value) = buffer.trim().parse::<i64>() {
                    if value < final_value {
                        eprintln!("Your answer was too low.");
                    } else if value > final_value {
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
