use std::{
    cell::{RefCell, RefMut},
    cmp::{max_by, Ord, Ordering, PartialOrd},
    collections::{btree_map::Entry, hash_map::DefaultHasher, BTreeMap},
    hash::{Hash, Hasher},
    process::exit,
};

use rand::{Rng, SeedableRng};

use rand_chacha::ChaChaRng;

// Triplet of scores for Alice, Bob and Charlie.
type Score = (i32, i32, i32);

// Enums used instead of modulo 3 for a tad more verbosity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Player {
    Alice,
    Bob,
    Charlie,
}

impl Player {
    // After Alice is Bob, after Bob is Charlie, after Charlie it's back to Alice...
    fn next_player(&self) -> Self {
        match *self {
            Self::Alice => Self::Bob,
            Self::Bob => Self::Charlie,
            Self::Charlie => Self::Alice,
        }
    }
}

// A struct to contain both the evaluation and principal variation
// to move the associated values together a bit easier.
// Comparison is still fully based on the evaluation score.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct EvalAndPV<const SIZE: usize> {
    pub eval: Score,
    pub pv: [Option<char>; SIZE],
}

impl<const SIZE: usize> Ord for EvalAndPV<SIZE> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.eval.cmp(&other.eval)
    }
}

impl<const SIZE: usize> PartialOrd for EvalAndPV<SIZE> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Some shortcuts used with RefCell for interior mutability,
// particularly when building up the game tree from puzzle input.
#[derive(Debug)]
struct GameState {
    value: RefCell<Score>,
    child_nodes: RefCell<BTreeMap<char, GameState>>,
}

impl GameState {
    // Non-leaf game states will be given a sentinel `value` field of `(0, 0, 0)`.
    fn new(value: Score) -> Self {
        GameState {
            value: RefCell::new(value),
            child_nodes: RefCell::new(BTreeMap::new()),
        }
    }

    // Recursively builds up the game tree, as each string entry
    // is a direct representation of all possible moves in all possible states.

    // Also currently not needed, but useful for demonstrating how it would be done by a participant.
    #[allow(dead_code)]
    fn update_path(&self, mut path: String, score: Score) {
        if !path.is_empty() {
            let m = path.remove(0);
            let mut mutable_borrow = self.child_nodes.borrow_mut();
            if let Entry::Vacant(e) = (*mutable_borrow).entry(m) {
                if path.is_empty() {
                    e.insert(GameState::new(score));
                } else {
                    e.insert(GameState::new((0, 0, 0)));
                    (*mutable_borrow).get(&m).unwrap().update_path(path, score);
                }
            } else {
                // Assume that no puzzle input is conflicting.
                (*mutable_borrow).get(&m).unwrap().update_path(path, score);
            }
        }
    }

    // Short-hand for getting all characters playable in a given game state.
    fn legal_moves(&self) -> Vec<char> {
        (*self.child_nodes.borrow()).keys().cloned().collect()
    }

    // To provide a collection of all possible next states
    // without referencing a local dereferenced variable,
    // we return a RefMut here.
    fn next_positions(&self) -> RefMut<BTreeMap<char, GameState>> {
        self.child_nodes.borrow_mut()
    }

    // Short-hand to retrieve and also mutate the game state score as needed.
    fn get_value(&self) -> RefMut<Score> {
        self.value.borrow_mut()
    }
}

/*
 * Puzzle prompt generation
 */

// Randomly generates a game tree, using a given RNG.
// To ensure the puzzle input is of an acceptable size,
// we repeatedly generate until the number of leaf nodes
// is within an acceptable range, determined by `lower` and `upper`.
// The parameter `p` will affect how likely it is for a branch to terminate into a leaf node early.
fn generate_gamestate<R: Rng>(
    rng: &mut R,
    depth: usize,
    p: f64,
    lower: i32,
    upper: i32,
) -> GameState {
    loop {
        let state = GameState::new((0, 0, 0));
        let number_of_leaf_nodes = generate_topology(rng, &state, depth, p);
        if number_of_leaf_nodes < lower || number_of_leaf_nodes > upper {
            continue;
        }
        let scores = score_list(rng, number_of_leaf_nodes);
        let mut iter = scores.into_iter();
        populate_with_scores(&state, &mut iter);
        return state;
    }
}

// Initialises every `GameState` instance with the sentinel `(0, 0, 0)` value for now,
// determining the number of child nodes in each state randomly.
fn generate_topology<R: Rng>(rng: &mut R, pos: &GameState, depth: usize, p: f64) -> i32 {
    if depth == 0 || rng.gen_bool(p) {
        return 1;
    }
    let mut sum: i32 = 0;
    let branches = rng.gen_range(0..5);
    for i in 0..=branches {
        let c = char::from_u32(i + 97).unwrap();
        let g = GameState::new((0, 0, 0));
        sum += generate_topology::<R>(rng, &g, depth - 1, p);
        (*pos.next_positions()).insert(c, g);
    }
    sum
}

// Helper function for `score_list`, but also usable for other operations
// in main, like shuffling the order of the puzzle input.
fn fisher_yates_shuffle<T: Clone, R: Rng>(rng: &mut R, vec: &mut [T]) {
    for i in (1..vec.len()).rev() {
        let j = rng.gen_range(0..=i);
        (vec[i], vec[j]) = (vec[j].clone(), vec[i].clone());
    }
}

// Each column of scores will be a permutation of the integers from
// 1 to the number of leaf nodes inclusive.
// We perform three shuffles of such a list, then zip them together to create the score triplets.
fn score_list<R: Rng>(rng: &mut R, size: i32) -> Vec<Score> {
    let mut alice_scores: Vec<i32> = (1..=size).collect();
    let mut bob_scores: Vec<i32> = (1..=size).collect();
    let mut charlie_scores: Vec<i32> = (1..=size).collect();

    fisher_yates_shuffle::<i32, R>(rng, &mut alice_scores);
    fisher_yates_shuffle::<i32, R>(rng, &mut bob_scores);
    fisher_yates_shuffle::<i32, R>(rng, &mut charlie_scores);

    alice_scores
        .iter()
        .zip(
            bob_scores
                .iter()
                .zip(charlie_scores.iter())
                .map(|(&b, &c)| (b, c)),
        )
        .map(|(&a, (b, c))| (a, b, c))
        .collect()
}

// After creating the topology, we need to populate each `GameState` instance with the
// score triplets we have generated randomly.
fn populate_with_scores(pos: &GameState, score_iter: &mut impl Iterator<Item = Score>) {
    let next_positions = pos.next_positions();
    if (*next_positions).is_empty() {
        (*pos.get_value()) = score_iter.next().unwrap();
        return;
    }
    for key in next_positions.keys() {
        populate_with_scores((*next_positions).get(key).unwrap(), score_iter);
    }
}

// Recursively collects each tree branch into a list of strings,
// which when shuffled will become the puzzle input.
fn display_paths_with_scores(pos: &GameState) -> Vec<String> {
    let next_positions = pos.next_positions();
    if (*next_positions).is_empty() {
        let (alice, bob, charlie) = *pos.get_value();
        return vec![format!(" {} {} {}", alice, bob, charlie)];
    }
    (*next_positions)
        .keys()
        .flat_map(|key| {
            display_paths_with_scores((*next_positions).get(key).unwrap())
                .iter()
                .map(|s| String::from(*key) + s)
                .collect::<Vec<_>>()
        })
        .collect()
}

/*
 * Puzzle solving
 */

// Reads a puzzle input and generates the game tree
// represented by the given string.
// Not actually needed here, but useful for verification of this program at some point.
#[allow(dead_code)]
fn read_puzzle_input(input_string: &str) -> GameState {
    let iterator = input_string.trim().split('\n').map(|line| {
        let row = line.split_whitespace().collect::<Vec<_>>();
        (
            row[0],
            (
                row[1].parse::<i32>().unwrap(),
                row[2].parse::<i32>().unwrap(),
                row[3].parse::<i32>().unwrap(),
            ),
        )
    });
    let root = GameState::new((0, 0, 0));
    for (path, score_triple) in iterator {
        root.update_path(path.to_string(), score_triple);
    }
    root
}

// The actual solving function itself.
// `SIZE` will be hardcoded in actuality,
// since it is always possible to determine ahead of time the maximum amount of moves playable.
fn search<const SIZE: usize>(pos: &GameState, depth: usize, player: Player) -> EvalAndPV<SIZE> {
    let legal_moves = pos.legal_moves();
    if legal_moves.is_empty() {
        return EvalAndPV {
            eval: *pos.get_value(),
            pv: [None; SIZE],
        };
    }
    let mut best = EvalAndPV {
        eval: (i32::MIN, i32::MIN, i32::MIN),
        pv: [None; SIZE],
    };
    for legal_move in legal_moves {
        let mut eval_and_pv = search::<SIZE>(
            (*pos.next_positions()).get(&legal_move).unwrap(),
            depth + 1,
            player.next_player(),
        );
        eval_and_pv.pv[depth] = Some(legal_move);
        best = max_by(eval_and_pv, best, |&ep1, &ep2| match player {
            Player::Alice => ep1.eval.0.cmp(&ep2.eval.0),
            Player::Bob => ep1.eval.1.cmp(&ep2.eval.1),
            Player::Charlie => ep1.eval.2.cmp(&ep2.eval.2),
        });
    }

    best
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 3);

    // Convert the string seed into a hash
    let mut default_hasher = DefaultHasher::new();
    args[2].hash(&mut default_hasher);
    let seed = default_hasher.finish();

    let mut rng = ChaChaRng::seed_from_u64(seed);

    // Seed is always given, and so we generate the game tree to start off with.
    let graph = generate_gamestate(&mut rng, 9, 0.2, 5000, 7000);

    match args[1].as_str() {
        "generate" => {
            // Create the string representation.
            let mut paths = display_paths_with_scores(&graph);

            // Shuffle the representation.
            fisher_yates_shuffle(&mut rng, &mut paths);

            for line in paths {
                println!("{line}");
            }
        }
        "validate" => {
            // First find the solution ourselves.
            let EvalAndPV {
                eval: (alice, bob, charlie),
                pv,
            } = search::<9>(&graph, 0, Player::Alice);

            let solution_string = format!(
                "{} {}",
                pv.iter()
                    .take_while(|c| c.is_some())
                    .map(|c| c.unwrap())
                    .collect::<String>(),
                (alice as u64) * (bob as u64) * (charlie as u64),
            );

            // Then we get the input.
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
