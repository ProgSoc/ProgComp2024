use std::{
    collections::HashMap,
    process::exit,
    sync::atomic::{AtomicUsize, Ordering},
};

use rand::{rngs::StdRng, Rng, SeedableRng};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    assert_eq!(args.len(), 3);

    let seed = args[2].parse::<u64>().unwrap();
    let mut rng = StdRng::seed_from_u64(seed);

    let length = rng.gen_range(20..=30);

    match args[1].as_str() {
        "generate" => {
            create_maze(length, &mut rng);
        }
        "validate" => {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();

            let input = buffer
                .trim()
                .parse::<usize>()
                .expect("Invalid input. Expected positive integer");

            if input == length {
                exit(0);
            } else {
                exit(1);
            }
        }
        _ => panic!(),
    }
}

type Graph = HashMap<NodeId, Node>;
type NodeId = usize;

struct Node {
    neighbors: Vec<NodeId>,
}

fn print_graph(graph: &Graph, id_shuffle_map: Option<&HashMap<NodeId, NodeId>>, start: NodeId, end: NodeId) {
    print_node(&start, &graph[&start], id_shuffle_map);
    print_node(&end, &graph[&end], id_shuffle_map);

    for (node_id, node) in graph.iter() {
        if *node_id == start || *node_id == end {
            continue;
        }

        print_node(node_id, node, id_shuffle_map);
    }
}

fn print_node(node_id: &NodeId, node: &Node, id_shuffle_map: Option<&HashMap<NodeId, NodeId>>) {
    let neighbours = node
        .neighbors
        .iter()
        .map(|&id| apply_shuffle_map(id, id_shuffle_map).to_string())
        .collect::<Vec<_>>()
        .join(", ");

    println!(
        "{}: {}",
        apply_shuffle_map(*node_id, id_shuffle_map),
        neighbours
    );
}

fn apply_shuffle_map(id: NodeId, id_shuffle_map: Option<&HashMap<NodeId, NodeId>>) -> NodeId {
    if let Some(id_shuffle_map) = id_shuffle_map {
        *id_shuffle_map.get(&id).unwrap()
    } else {
        id
    }
}

static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn create_node(graph: &mut Graph) -> NodeId {
    let id = ID_COUNTER.fetch_add(1, Ordering::SeqCst);
    graph.insert(id, Node { neighbors: vec![] });
    id
}

fn link_nodes(graph: &mut Graph, a: NodeId, b: NodeId) {
    graph.get_mut(&a).unwrap().neighbors.push(b);
    graph.get_mut(&b).unwrap().neighbors.push(a);
}

fn create_chain(graph: &mut Graph, length: usize) -> Vec<NodeId> {
    let mut node_ids = vec![];
    for _ in 0..length {
        let node_id = create_node(graph);
        if let Some(&last_node) = node_ids.last() {
            link_nodes(graph, last_node, node_id);
        }
        node_ids.push(node_id);
    }
    node_ids
}

fn create_web(graph: &mut Graph, rng: &mut StdRng) -> Vec<NodeId> {
    let node_count = rng.gen_range(10..=20);
    let mut node_ids = vec![];

    for _ in 0..node_count {
        let node_id = create_node(graph);
        node_ids.push(node_id);
    }

    assert!(node_ids.len() >= 2);

    for node_id in &node_ids {
        let neighbor_count = rng.gen_range(1..=2);
        let mut node_ids = node_ids.clone();
        node_ids.retain(|&id| id != *node_id);

        for _ in 0..neighbor_count {
            let index = rng.gen_range(0..node_ids.len());

            let neighbor_id = node_ids
                .splice(index..=index, vec![])
                .next()
                .clone()
                .unwrap();

            link_nodes(graph, *node_id, neighbor_id);
        }
    }

    node_ids
}

fn swap_vec_elements<T: Clone>(vec: &mut Vec<T>, a: usize, b: usize) {
    let tmp = vec[a].clone();
    vec[a] = vec[b].clone();
    vec[b] = tmp;
}

fn create_id_shuffle_map(
    graph: &Graph,
    rng: &mut StdRng,
    start: NodeId,
    end: NodeId,
) -> HashMap<NodeId, NodeId> {
    let mut map = HashMap::new();
    map.insert(start, 0);
    map.insert(end, 1);
   
    let mut node_ids = graph.keys().cloned().collect::<Vec<_>>();
    node_ids.retain(|&id| id != start && id != end);

    let mut shuffled_node_ids = node_ids.clone();
    for i in 0..shuffled_node_ids.len() {
        let j = rng.gen_range(0..shuffled_node_ids.len());
        swap_vec_elements(&mut shuffled_node_ids, i, j);
    }

    for (node_id, shuffled_node_id) in node_ids.iter().zip(shuffled_node_ids.iter()) {
        map.insert(*node_id, *shuffled_node_id);
    }

    map
}

fn create_maze(route_length: usize, rng: &mut StdRng) {
    assert!(route_length > 1);

    let mut graph = Graph::new();

    let chain_node_ids = create_chain(&mut graph, route_length);

    for chain_node_id in &chain_node_ids {
        let mut web_node_ids = create_web(&mut graph, rng);
        let links = rng.gen_range(1..web_node_ids.len());

        for _ in 0..links {
            let index = rng.gen_range(0..web_node_ids.len());
            let web_node_id = web_node_ids
                .splice(index..=index, vec![])
                .next()
                .clone()
                .unwrap();
            link_nodes(&mut graph, *chain_node_id, web_node_id);
        }
    }

    let start = *chain_node_ids.first().unwrap();
    let end = *chain_node_ids.last().unwrap();
    assert_ne!(start, end);

    let id_shuffle_map = create_id_shuffle_map(&graph, rng, start, end);

    print_graph(&graph, Some(&id_shuffle_map), start, end);
}
