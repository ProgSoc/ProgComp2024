use std::{
    cmp::{max, Ord, Ordering, PartialOrd},
    collections::{hash_map::DefaultHasher, BinaryHeap, HashMap},
    hash::{Hash, Hasher},
    process::exit,
};

use rand::{seq::SliceRandom, Rng, SeedableRng};

use rand_chacha::ChaChaRng;

type NodeID = [char; 3];

type Graph = HashMap<NodeID, Vec<(NodeID, u32)>>;

fn key_string(node: NodeID) -> String {
    format!("{}{}{}", node[0], node[1], node[2])
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct State {
    cost: u32,
    position: NodeID,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.position.cmp(&self.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn min_cost(graph: &Graph, start: NodeID, end: NodeID) -> Option<u32> {
    let mut costs: HashMap<NodeID, u32> = HashMap::new();
    for &node_id in graph.keys() {
        costs.insert(node_id, u32::MAX);
    }
    let mut open: BinaryHeap<State> = BinaryHeap::new();
    open.push(State {
        cost: 0,
        position: start,
    });
    while let Some(State { cost, position }) = open.pop() {
        if position == end {
            return Some(cost);
        }
        if cost > *costs.get(&position).unwrap() {
            continue;
        }
        for &(neighbour, edge_cost) in graph.get(&position).unwrap().iter() {
            if cost + edge_cost < *costs.get(&neighbour).unwrap() {
                open.push(State {
                    cost: cost + edge_cost,
                    position: neighbour,
                });
                costs.insert(neighbour, cost + edge_cost);
            }
        }
    }
    None
}

fn max_cost(
    graph: &Graph,
    prev_cost: u32,
    path: &mut Vec<NodeID>,
    start: NodeID,
    end: NodeID,
) -> u32 {
    if start == end {
        return prev_cost;
    }
    graph
        .get(&start)
        .unwrap()
        .iter()
        .fold(0, |acc, &(node, cost)| {
            if !path.contains(&node) {
                path.push(node);
                let result = max_cost(graph, prev_cost + cost, path, node, end);
                path.pop();
                max(acc, result)
            } else {
                acc
            }
        })
}

fn node_from_integer(num: u32) -> NodeID {
    let c1 = (num / 676) % 26;
    let c2 = (num / 26) % 26;
    let c3 = num % 26;
    [
        char::from_u32(c1 + 97).unwrap(),
        char::from_u32(c2 + 97).unwrap(),
        char::from_u32(c3 + 97).unwrap(),
    ]
}

fn generate_graph<R: Rng>(rng: &mut R, node_list: Vec<NodeID>) -> Graph {
    loop {
        let mut graph: Graph = node_list
            .clone()
            .into_iter()
            .map(|node| (node, Vec::new()))
            .collect();

        for node in &node_list {
            graph.get_mut(node).unwrap().append(
                &mut (node_list.choose_multiple(rng, 4))
                    .map(|&x| (x, rng.gen_range(1000..=5000)))
                    .collect::<Vec<_>>(),
            );
        }

        if min_cost(&graph, ['a', 'a', 'a'], ['z', 'z', 'z']).is_some() {
            return graph;
        }
    }
}

fn graph_string(g: &Graph, node_list: &[NodeID]) -> Vec<String> {
    node_list
        .iter()
        .map(|&node| {
            format!(
                "{} -> {}",
                key_string(node),
                g.get(&node)
                    .unwrap()
                    .iter()
                    .map(|&(n, c)| { format!("{}: {}", key_string(n), c) })
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })
        .collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    assert_eq!(args.len(), 3);

    // Convert the string seed into a hash
    let mut default_hasher = DefaultHasher::new();
    args[2].hash(&mut default_hasher);
    let seed = default_hasher.finish();

    let mut rng = ChaChaRng::seed_from_u64(seed);

    // Generate random graph.
    let mut node_list: Vec<NodeID> = (1..17575)
        .collect::<Vec<u32>>()
        .choose_multiple(&mut rng, 20)
        .map(|&x| node_from_integer(x))
        .collect();

    let start = ['a', 'a', 'a'];
    let end = ['z', 'z', 'z'];

    node_list.insert(0, start);
    node_list.push(end);

    let g = generate_graph(&mut rng, node_list.clone());

    match args[1].as_str() {
        "generate" => {
            for line in graph_string(&g, &node_list) {
                println!("{line}");
            }
        }
        "validate" => {
            // First find the solution ourselves.
            let min = min_cost(&g, start, end).unwrap();
            let max = max_cost(&g, 0, &mut vec![start], start, end);
            let solution_string = (min * max).to_string();

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
