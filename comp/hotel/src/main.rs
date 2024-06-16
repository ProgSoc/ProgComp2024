use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
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

    let problem = generate_problem(seed);

    match args[1].as_str() {
        "generate" => {
            print_stays(&problem);
        }
        "validate" => {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();

            let colouring = parse_colouring(buffer).graceful_unwrap();

            if colouring.len() < problem.len() {
                eprintln!("Too few allocations.");
                exit(1);
            }

            if colouring.len() > problem.len() {
                eprintln!("Too many allocations.");
                exit(1);
            }

            const MAX_ROOM_NUMBER: usize = 49;

            if *colouring.iter().map(|(_, room)| room).max().unwrap() > MAX_ROOM_NUMBER {
                eprintln!("Too many rooms used.");
                exit(1);
            }

            let mut graph = create_graph(&problem);

            apply_colouring(&mut graph, colouring);

            if has_colouring_conflicts(&graph) {
                eprintln!("Some guests where assigned the same room at the same time.");
                exit(1);
            }

            if !can_clean(&graph).is_yes() {
                eprintln!("Rooms could not be cleaned in time.");
                exit(1);
            }

            exit(0);
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

type OccupancyId = usize;
type Graph = Vec<Occupancy>;
type RoomNumber = usize;

#[derive(Clone, Debug)]
struct Occupancy {
    room: Option<RoomNumber>,
    duration: Period,

    /// Occupancies that cannot be allocated the same room
    neighbours: HashSet<OccupancyId>,
    ignore: bool,
}

static CLEANING_TIME: Second = 1800;

type Second = i64;

#[derive(Clone, Debug)]
struct Period {
    start: Second,
    end: Second,
}

impl Period {
    fn conincides(&self, other: &Period) -> bool {
        (self.start <= other.end && self.start >= other.start)
            || (self.end <= other.end && self.end >= other.start)
    }

    fn distance(&self, other: &Period) -> Second {
        if self.conincides(other) {
            return 0;
        }

        if other.start > self.end {
            return other.start - self.end;
        } else {
            return other.end - self.start;
        }
    }
}

fn create_graph(stays: &Vec<Period>) -> Vec<Occupancy> {
    let mut graph: Graph = stays
        .into_iter()
        .map(|d| Occupancy {
            room: None,
            duration: d.clone(),
            neighbours: HashSet::new(),
            ignore: false,
        })
        .collect::<Graph>();

    for i in 0..graph.len() {
        for j in 0..graph.len() {
            if i == j {
                continue;
            }

            if graph[i].duration.distance(&graph[j].duration).abs() <= CLEANING_TIME {
                graph[i].neighbours.insert(j);
                graph[j].neighbours.insert(i);
            }
        }
    }

    graph
}

fn colour_graph(graph: &mut Graph) -> usize {
    let mut k = 0;

    for i in 0..graph.len() {
        let mut room: RoomNumber = 0;

        if graph[i].ignore {
            continue;
        }

        loop {
            let mut found = true;
            for nei_idx in &graph[i].neighbours {
                assert_ne!(*nei_idx, i);

                if graph[*nei_idx].ignore {
                    continue;
                }

                if let Some(nei_room) = graph[*nei_idx].room {
                    if nei_room == room {
                        found = false;
                        room += 1;
                    }
                }
            }

            if found {
                break;
            }
        }

        graph[i].room = Some(room);
        k = k.max(room);
    }

    k += 1;

    return k;
}

#[derive(Clone, Debug)]
struct Job {
    avalible_time_to_complete: Period,
    occupant: OccupancyId,
    prior_occupant: OccupancyId,

    #[allow(unused)] // This is useful for debugging
    room: RoomNumber,
}

#[derive(Debug)]
enum CanClean {
    Yes,
    No {
        troublesome_occupancy: OccupancyId,
        prior_occupancy: OccupancyId,
    },
}

impl CanClean {
    fn is_yes(&self) -> bool {
        match self {
            CanClean::Yes => true,
            _ => false,
        }
    }
}

fn can_clean(graph: &Graph) -> CanClean {
    let mut jobs: Vec<Job> = vec![];
    let mut last_in_room: [Second; 300] = [0; 300];
    let mut last_occupant_in_room: [i32; 300] = [-1; 300];

    let mut numbered_graph: Vec<(usize, Occupancy)> =
        graph.clone().into_iter().enumerate().collect();

    numbered_graph.sort_by_key(|(_, o)| o.duration.start);

    for (id, occ) in numbered_graph {
        if occ.ignore {
            continue;
        }

        assert!(occ.room.is_some());

        let room = occ.room.unwrap();

        if last_in_room[room] != 0 {
            let period = Period {
                start: last_in_room[room],
                end: occ.duration.start,
            };

            assert!(period.end - period.start > CLEANING_TIME);

            let prior_occupant = last_occupant_in_room[room];

            assert_ne!(prior_occupant, -1);

            jobs.push(Job {
                avalible_time_to_complete: period,
                occupant: id,
                prior_occupant: prior_occupant as usize,
                room,
            });
        }

        last_in_room[room] = occ.duration.end;
        last_occupant_in_room[room] = id as i32;
    }

    const STAFF_COUNT: usize = 5;

    let mut staff: Vec<Vec<Job>> = vec![Vec::new(); STAFF_COUNT];

    // TODO: try all possible orderings of job allocations
    for job in jobs {
        if !try_allocate(&mut staff, &job) {
            return CanClean::No {
                troublesome_occupancy: job.occupant,
                prior_occupancy: job.prior_occupant,
            };
        }
    }

    CanClean::Yes
}

fn try_allocate(staff: &mut Vec<Vec<Job>>, job: &Job) -> bool {
    // TODO: try all possible orderings of staff allocations
    for member in staff {
        let mut new_job_list = member.clone();
        new_job_list.push(job.clone());
        if can_fit_job(new_job_list) {
            member.push(job.clone());
            return true;
        }
    }

    false
}

/// Attempts to find timeslots for each job to be completed
fn can_fit_job(jobs: Vec<Job>) -> bool {
    let allocations = jobs
        .iter()
        .map(|j| j.avalible_time_to_complete.start)
        .collect::<Vec<Second>>();

    __can_fit_job(allocations, &jobs, 0, 0)
}

fn __can_fit_job(allocations: Vec<Second>, jobs: &Vec<Job>, depth: usize, start: usize) -> bool {
    if depth > allocations.len() {
        return false;
    }

    if valid_job_allocations(&allocations) {
        return true;
    }

    'a: for i in 0..allocations.len() {
        if !has_conflicts(i, &allocations) {
            continue;
        }

        let mut allocations = allocations.clone();

        // Move i to next available time.
        match next_avalible_time(i, allocations.clone(), &jobs[i].avalible_time_to_complete) {
            None => continue 'a,
            Some(time) => {
                assert!(
                    time >= jobs[i].avalible_time_to_complete.start
                        && time + CLEANING_TIME < jobs[i].avalible_time_to_complete.end
                );

                allocations[i] = time;
            }
        }

        if __can_fit_job(allocations, jobs, depth + 1, start + 1) {
            return true;
        }
    }

    false
}

fn has_conflicts(index: usize, allocations: &Vec<Second>) -> bool {
    for i in 0..allocations.len() {
        if i == index {
            continue;
        }

        if (allocations[i] - allocations[index]).abs() < CLEANING_TIME {
            return true;
        }
    }

    return false;
}

fn next_avalible_time(
    ignore: usize,
    mut allocations: Vec<Second>,
    period: &Period,
) -> Option<Second> {
    allocations.remove(ignore);

    for allocation in &allocations {
        let guess = allocation + CLEANING_TIME;

        if guess < period.start || guess + CLEANING_TIME >= period.end {
            continue;
        }

        let mut new_allocations = allocations.clone();
        new_allocations.push(guess);
        if valid_job_allocations(&allocations) {
            return Some(guess);
        }
    }

    None
}

fn valid_job_allocations(allocations: &Vec<Second>) -> bool {
    for i in 0..allocations.len() {
        for j in 0..allocations.len() {
            if i == j {
                continue;
            }

            if (allocations[i] - allocations[j]).abs() <= CLEANING_TIME {
                return false;
            }
        }
    }

    true
}

fn gen(seed: u64) -> Vec<Period> {
    let mut rng = StdRng::seed_from_u64(seed);

    let mut occs = vec![];

    for _ in 0..120 {
        let len = rng.gen_range(7_000..25_000);

        let start = rng.gen_range(0..100_000);

        occs.push(Period {
            start,
            end: start + len,
        });
    }

    occs
}

fn print_stays(stays: &Vec<Period>) {
    for stay in stays {
        println!("{}, {}", stay.start, stay.end);
    }
}

fn decolour(graph: &mut Graph) {
    for occ in graph {
        occ.room = None;
    }
}

fn try_solve(graph: &mut Graph) -> usize {
    let k = colour_graph(graph);

    let cleanable = can_clean(&graph);

    match cleanable {
        CanClean::Yes => k,
        CanClean::No {
            troublesome_occupancy,
            prior_occupancy,
        } => {
            decolour(graph);

            // Try again making sure that the occupancies that prevented cleaning
            // are allocated different rooms.
            graph[troublesome_occupancy]
                .neighbours
                .insert(prior_occupancy);
            graph[prior_occupancy]
                .neighbours
                .insert(troublesome_occupancy);

            return try_solve(graph);
        }
    }
}

fn generate_problem(seed: u64) -> Vec<Period> {
    let guests = gen(seed);

    let mut graph: Graph = create_graph(&guests);

    let rooms_needed = try_solve(&mut graph);

    const MIN_ROOMS: usize = 15;
    const MAX_ROOMS: usize = 45;

    if rooms_needed < MIN_ROOMS || rooms_needed > MAX_ROOMS {
        return generate_problem(seed + 1);
    }

    guests
}

fn parse_colouring(colouring: String) -> Result<HashMap<OccupancyId, RoomNumber>, String> {
    let mut map = HashMap::new();

    for (occ_id, room_allocation) in colouring.split(",").enumerate() {
        let room = room_allocation
            .trim()
            .parse::<RoomNumber>()
            .map_err(|_| "Expected numbers.")?;

        assert!(map.get(&occ_id).is_none());

        map.insert(occ_id, room);
    }

    Ok(map)
}

fn apply_colouring(graph: &mut Graph, colouring: HashMap<OccupancyId, RoomNumber>) {
    for (id, occ) in graph.iter_mut().enumerate() {
        if colouring.get(&id).is_none() {
            panic!("Not all guests allocated");
        }

        occ.room = Some(*colouring.get(&id).unwrap());
    }
}

fn has_colouring_conflicts(graph: &Graph) -> bool {
    for i in 0..graph.len() {
        for j in &graph[i].neighbours {
            if graph[i].room == graph[*j].room {
                return true;
            }
        }
    }

    false
}
