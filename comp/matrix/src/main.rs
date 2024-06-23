use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    process::exit,
};

use rand::{Rng, SeedableRng};

use rand_chacha::ChaChaRng;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    assert_eq!(args.len(), 3);

    let mut s = DefaultHasher::new();
    args[2].hash(&mut s);
    let seed = s.finish();

    let mut rng = ChaChaRng::seed_from_u64(seed);

    const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.";

    let mut strings = LOREM_IPSUM
        .split_whitespace()
        .map(|s| s.chars().filter(|c| c.is_alphabetic()).collect::<String>())
        .collect::<Vec<String>>();

    for _ in 0..4 {
        strings.remove(rng.gen_range(0..strings.len()));
    }

    match args[1].as_str() {
        "generate" => {
            let height = 40;
            let width = 90;

            let code = draw_code(strings, height, width, &mut rng);

            print_code(code);
        }
        "validate" => {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();

            let avg_word_length =
                strings.iter().map(|s| s.len()).sum::<usize>() as f64 / strings.len() as f64;

            let input_avg_len = buffer
                .trim()
                .parse::<f64>()
                .graceful_expect("Expected a number.");

            if (avg_word_length - input_avg_len).abs() < 0.1 {
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

fn draw_code<R: Rng>(
    strings: Vec<String>,
    height: usize,
    width: usize,
    rng: &mut R,
) -> Vec<Vec<String>> {
    // Append spaces to the end of each string. Messy but simple way to ensure that stings
    // have at least one space between them.
    let strings = strings
        .into_iter()
        .map(|s| s + " ")
        .collect::<Vec<String>>();

    let height = height + 1; // Spaces at end of each string

    // Initialise blank 2D array
    let mut result = vec![vec![" ".to_string(); width]; height];

    for string in strings {
        // Pick initial position to try to draw the string to
        let mut x = rng.gen_range(0..width);
        let mut y = rng.gen_range(0..=height - string.len());

        // Check that the vertical range is vacant
        let (x_original, y_original) = (x, y);
        while !vert_range_is_vacant(&result, x, y, string.len()) {
            // If not, try 1 character below. If at bottom, try next column.

            y += 1;
            if y + string.len() >= result.len() {
                y = 0;
                x = (x + 1) % width;

                // Retry the random y at the new x so things don't bunch up around the top.
                // Otherwise just start from the top and check the entire column.
                if vert_range_is_vacant(&result, x, y_original, string.len()) {
                    y = y_original;
                }
            }

            if x == x_original && y == y_original {
                panic!("Could not find a place to put the string.");
            }
        }

        for c in string.chars() {
            assert!(y < result.len());
            assert!(x < result[y].len());

            result[y][x] = c.to_string();
            y += 1;
        }
    }

    result.pop(); // Remove the last row of spaces

    result
}

fn vert_range_is_vacant(code: &Vec<Vec<String>>, x: usize, y: usize, len: usize) -> bool {
    if y != 0 && code[y - 1][x] != " " {
        return false;
    }

    (y..y + len).all(|y| position_is_vacant(code, x, y))
}

fn position_is_vacant(code: &Vec<Vec<String>>, x: usize, y: usize) -> bool {
    code[y][x] == " "
}

fn print_code(code: Vec<Vec<String>>) {
    for row in code {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}
