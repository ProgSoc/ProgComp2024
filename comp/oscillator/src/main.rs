#![allow(non_snake_case)]

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    ops::Range,
    process::exit,
};

use rand::{Rng, SeedableRng};

use rand_chacha::ChaChaRng;

mod units;

use units::*;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    assert_eq!(args.len(), 3);

    let mut s = DefaultHasher::new();
    args[2].hash(&mut s);
    let seed = s.finish();

    let mut system = generate_system(seed);

    match args[1].as_str() {
        "generate" => {
            print_system(&system);
        }
        "validate" => {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();

            let input = buffer
                .trim()
                .parse::<f64>()
                .graceful_expect("Invalid input. Expected number.");

            system = simulate(system);
            let solution = system.bodies[0].x.0;

            if (input - solution).abs() <= 0.05 {
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

type BodyId = usize;

#[derive(Clone)]
struct Body {
    id: BodyId,
    x: Position,
    v: Velocity,
    m: Mass,
}

#[derive(Clone, Debug)]
struct Spring {
    k: SpringConstant,
    rest_length: f64,
    joint1: SpringJoint,
    joint2: SpringJoint,
}

#[derive(Clone, PartialEq, Debug)]
enum SpringJoint {
    Fixed(Position),
    Body(BodyId),
}

#[derive(Clone)]
struct System {
    bodies: Vec<Body>,
    springs: Vec<Spring>,
}

impl SpringJoint {
    fn position(&self, system: &System) -> Position {
        match self {
            SpringJoint::Fixed(x) => *x,
            SpringJoint::Body(id) => system.bodies[*id].x,
        }
    }
}

impl Spring {
    fn force(&self, system: &System) -> Force {
        let x1 = self.joint1.position(system);
        let x2 = self.joint2.position(system);
        let Δx = x2 - x1;
        self.k * (Position(self.rest_length) - Δx)
    }
}

impl Body {
    /// Loops through all springs, if the spring is connected to the body,
    /// it's force is added.
    fn acceleration(&self, system: &System) -> Acceleration {

        let mut F = Force(0.0);

        for spring in system.springs.iter() {
            if spring.joint1 == SpringJoint::Body(self.id) {
                let direction = (spring.joint2.position(system) - self.x).0.signum();
                F = F + spring.force(system) * Force(-direction);
            }

            if spring.joint2 == SpringJoint::Body(self.id) {
                let direction = (spring.joint1.position(system) - self.x).0.signum();
                F = F + spring.force(system) * Force(-direction);
            }
        }

        F / self.m
    }

    fn apply_euler_step(&mut self, system: &System, Δt: Time) {
        let a = self.acceleration(system);
        self.x = self.x + self.v * Δt;
        self.v = self.v + a * Δt;
    }
}

fn eulers_step(system: System, Δt: Time) -> System {
    let mut new_system = system.clone();

    for i in 0..new_system.bodies.len() {
        new_system.bodies[i].apply_euler_step(&system, Δt);
    }

    new_system
}

/// Create system as in diagram.
fn generate_system(seed: u64) -> System {
    let mut rng = ChaChaRng::seed_from_u64(seed);

    let body_count = rng.gen_range(5..8);

    const SPACING: f64 = 5.0;
    const SPRING_CONSTANT_RANGE: Range<f64> = 0.1..1.0;
    const SPRING_REST_LENGTH: f64 = 5.0;

    let bodies: Vec<Body> = (0..body_count)
        .map(|i| Body {
            id: i,
            x: Position((i + 1) as f64 * SPACING + rng.gen_range(-1.0..1.0)),
            v: Velocity(rng.gen_range(-0.01..0.01)),
            m: Mass(rng.gen_range(0.1..0.5)),
        })
        .collect();

    let mut springs: Vec<Spring> = (0..bodies.len())
        .collect::<Vec<BodyId>>()
        .windows(2)
        .map(|pair| Spring {
            k: SpringConstant(rng.gen_range(SPRING_CONSTANT_RANGE.clone())),
            rest_length: SPRING_REST_LENGTH,
            joint1: SpringJoint::Body(pair[0]),
            joint2: SpringJoint::Body(pair[1]),
        })
        .collect();

    let left_wall = SpringJoint::Fixed(Position(0.0));

    springs.insert(
        0,
        Spring {
            k: SpringConstant(rng.gen_range(SPRING_CONSTANT_RANGE.clone())),
            rest_length: SPRING_REST_LENGTH,
            joint1: left_wall,
            joint2: SpringJoint::Body(0),
        },
    );

    System { bodies, springs }
}

fn print_system(system: &System) {
    println!("x, v, m, k");
    for (body, spring) in system.bodies.iter().zip(system.springs.iter()) {
        println!("{}, {}, {}, {}", body.x.0, body.v.0, body.m.0, spring.k.0);
    }
}

fn simulate(mut system: System) -> System {
    // For smaller h, the simulation is more accurate but slower.
    let h = Time(0.00001);

    let t_final = Time(5.0);

    let mut t = Time(0.0);

    let mut times: Vec<f64> = vec![];
    let mut positions: Vec<Vec<f64>> = vec![vec![]; system.bodies.len()];

    loop {
        t = t + h;

        if t >= t_final {
            break;
        }

        system = eulers_step(system, h);

        times.push(t.0);
        for (i, body) in system.bodies.iter().enumerate() {
            positions[i].push(body.x.0);
        }
    }

    // println!("{:?}", system.bodies[0].x.0);

    // plot(times, positions);

    system
}

use rustplotlib::Figure;

#[allow(dead_code)]
fn plot(times: Vec<f64>, positions: Vec<Vec<f64>>) {
    use rustplotlib::{Axes2D, Line2D};

    let mut ax = Axes2D::new();

    for (i, body_positions) in positions.iter().enumerate() {
        ax = ax.add(
            Line2D::new(i.to_string().as_str())
                .data(times.as_slice(), body_positions.as_slice())
                .linewidth(1.0),
        );
    }

    let fig = Figure::new().subplots(1, 1, vec![Some(ax)]);

    use rustplotlib::backend::Matplotlib;
    use rustplotlib::Backend;
    let mut mpl = Matplotlib::new().unwrap();
    mpl.set_style("ggplot").unwrap();

    fig.apply(&mut mpl).unwrap();

    mpl.savefig("plot.png").unwrap();
    mpl.wait().unwrap();
}
