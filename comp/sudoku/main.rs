use std::{collections::HashSet, process::exit};

use rand::{rngs::StdRng, Rng, SeedableRng};

// FIXME: Some seeds take a very long time to generate.
// slow seeds: 999211, 945333

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    assert_eq!(args.len(), 3);

    let seed = args[2].parse::<u64>().unwrap();

    const GIVENS: usize = 48;
    const MIN_PERMS: usize = 5;
    const MAX_PERMS: usize = 15;

    let gs = new_game(GIVENS, MIN_PERMS, MAX_PERMS, seed);

    match args[1].as_str() {
        "generate" => {
            print_game_state(&gs);
        }
        "validate" => {
            let perms = permuatations(&gs, None).len();

            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();

            let input = buffer
                .trim()
                .parse::<usize>()
                .expect("Invalid input. Expected positive integer");

            if input == perms {
                exit(0);
            } else {
                exit(1);
            }
        }
        _ => panic!(),
    }
}

type CellId = usize;
type GameState = Vec<Cell>;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Cell {
    value: Option<u8>,
}

fn position(cell_id: CellId) -> (usize, usize) {
    (cell_id % 9, cell_id / 9)
}

fn graph_neighbours(cell_id: CellId) -> Vec<CellId> {
    let (x, y) = position(cell_id);

    let mut neighbours = vec![];

    let (segment_x, segment_y) = (x / 3, y / 3);

    neighbours.extend((0..9).map(|i| vec![i * 9 + x, y * 9 + i]).flatten());

    for i in 0..3 {
        for j in 0..3 {
            neighbours.push((segment_y * 3 + i) * 9 + segment_x * 3 + j);
        }
    }

    neighbours.into_iter().filter(|&id| id != cell_id).collect()
}

fn valid_insertion(gs: &GameState, cell_id: CellId, value: u8) -> bool {
    for neighbour in graph_neighbours(cell_id) {
        if let Some(neighbour_value) = gs[neighbour].value {
            if neighbour_value == value {
                return false;
            }
        }
    }
    return true;
}

fn possible_insertions(gs: &GameState, cell_id: CellId) -> Vec<u8> {
    (1..=9)
        .filter(|&value| valid_insertion(gs, cell_id, value))
        .collect()
}

fn permuatations(gs: &GameState, max_perms: Option<usize>) -> HashSet<GameState> {
    let mut perms = HashSet::new();
    __permutations(&gs, 0, &mut perms, max_perms);
    perms
}

fn __permutations(
    gs: &GameState,
    start_id: usize,
    perms: &mut HashSet<GameState>,
    max_perms: Option<usize>,
) {
    if let Some(max_perms) = max_perms {
        if perms.len() >= max_perms {
            return;
        }
    }

    let mut id = start_id;

    while gs[id].value.is_some() {
        id += 1;

        if id == gs.len() {
            perms.insert(gs.clone());
            return;
        }
    }

    for &value in possible_insertions(gs, id).iter() {
        let mut new_gs = gs.clone();
        new_gs[id].value = Some(value);

        __permutations(&new_gs, id, perms, max_perms);
    }
}

fn print_game_state(gs: &GameState) {
    for row in 0..9 {
        println!(
            "{}",
            gs[row * 9..(row + 1) * 9]
                .iter()
                .map(|cell| cell
                    .value
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "0".to_string()))
                .collect::<Vec<String>>()
                .join(", ")
        );
    }
}

fn random_vacant_position(gs: &GameState, rng: &mut StdRng) -> CellId {
    let mut loop_limit = 100;
    loop {
        let id = rng.gen::<usize>() % 81;

        if gs[id].value.is_none() {
            return id;
        }

        loop_limit -= 1;
        if loop_limit == 0 {
            break;
        }
    }

    for (i, cell) in gs.iter().enumerate() {
        if cell.value.is_none() {
            return i;
        }
    }

    panic!("No vacant position found.");
}

fn blank_game() -> GameState {
    vec![Cell { value: None }; 81]
}

fn new_game(givens: usize, min_perms: usize, max_perms: usize, seed: u64) -> GameState {
    let mut gs = blank_game();

    // This is called recursively if the last seed failed to generate
    // a desirable game so we increment the seed here.
    let seed = seed + 1;
    let mut rng = StdRng::seed_from_u64(seed);

    // Inserts some random values.

    const INITIAL_GIVENS: usize = 7;

    for _ in 0..INITIAL_GIVENS {
        let id = random_vacant_position(&gs, &mut rng);

        let possible = possible_insertions(&gs, id);

        if possible.is_empty() {
            return new_game(givens, min_perms, max_perms, seed);
        }

        let value = rng.gen::<u8>() % possible.len() as u8;
        gs[id].value = Some(possible[value as usize]);
    }

    // Attempts to find a solution with inserted values.

    let solution = permuatations(&gs, Some(1));

    if solution.len() == 0 {
        return new_game(givens, min_perms, max_perms, seed);
    }

    // Copies several random values from the solution to create the final game state.

    let solution = solution.into_iter().next().unwrap();
    let mut final_gs = blank_game();

    for _ in 0..givens {
        let id = random_vacant_position(&gs, &mut rng);
        final_gs[id].value = solution[id].value;
    }

    // Checks if the number of permutations is within the desired range.

    let num_perms = permuatations(&final_gs, Some(max_perms + 1)).len();

    if num_perms < min_perms || num_perms > max_perms {
        return new_game(givens, min_perms, max_perms, seed);
    }

    final_gs
}
