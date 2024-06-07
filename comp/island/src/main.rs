type Elevation = f64;
type Terrain = Vec<Vec<Elevation>>;

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    process::exit,
};

use noise::{
    core::perlin_surflet::perlin_surflet_2d, permutationtable::PermutationTable, utils::*,
};

use rand::{rngs::StdRng, Rng, SeedableRng};

mod solve;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    assert_eq!(args.len(), 3);

    let mut s = DefaultHasher::new();
    args[2].hash(&mut s);
    let seed = s.finish();

    let terrain = generate_problem(seed);

    match args[1].as_str() {
        "generate" => {
            print_terrain(&terrain);
        }
        "validate" => {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();

            let land_masses = solve::recurive_islands(&terrain);

            let area = land_masses.iter().map(|lm| lm.area()).sum::<usize>();

            let input = buffer
                .trim()
                .parse::<usize>()
                .graceful_expect("Expected a positive integer.");

            if area == input {
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

fn print_terrain(terrain: &Terrain) {
    // for row in terrain {
    //     for &elevation in row {
    //         print!("{}", if elevation > 0.0 { "X" } else { " " });
    //     }
    //     println!();
    // }

    for row in terrain {
        println!(
            "{}",
            row.iter()
                .map(|elevation| format!("{:.2}", elevation))
                .collect::<Vec<String>>()
                .join(", ")
        );
    }
}

fn blank_terrain(width: usize, height: usize) -> Terrain {
    vec![vec![0.0; width]; height]
}

fn gaussian(x: f64, mean: f64, sigma: f64) -> f64 {
    let a = 1.0 / (sigma * (2.0 * std::f64::consts::PI).sqrt());
    let b = -0.5 * ((x - mean) / sigma).powi(2);
    a * b.exp()
}

#[derive(Clone, Copy, Debug)]
pub struct Point {
    x: f64,
    y: f64,
}

fn gaussian_2d(p: Point, mean: Point, sigma: f64) -> f64 {
    let distance = ((p.x - mean.x).powi(2) + (p.y - mean.y).powi(2)).sqrt();
    gaussian(distance, 0.0, sigma)
}

fn elevation_at(terrain: &Terrain, x: f64, y: f64) -> Option<Elevation> {
    if x < 0. || y < 0. || x >= terrain.len() as f64 || y >= terrain[0].len() as f64 {
        return None;
    }

    Some(terrain[x as usize][y as usize])
}

fn add_terrain(a: &mut Terrain, b: &Terrain, p: Point) {
    for y in 0..b.len() {
        for x in 0..b[0].len() {
            let a_x = x + p.x as usize;
            let a_y = y + p.y as usize;

            if a_x >= a[0].len() || a_y >= a.len() {
                continue;
            }

            a[a_y][a_x] += b[y][x];
        }
    }
}

fn multiply_terrain(a: &mut Terrain, b: &Terrain) {
    for y in 0..a.len() {
        for x in 0..a[0].len() {
            a[y][x] *= b[y][x];
        }
    }
}

fn scale_terrain(terrain: &mut Terrain, scale: f64) {
    terrain.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|elevation| {
            *elevation *= scale;
        });
    });
}

fn offset_terrain(terrain: &mut Terrain, offset: f64) {
    terrain.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|elevation| {
            *elevation += offset;
        });
    });
}

fn gauss_blur(terrain: Terrain) -> Terrain {
    let mut result = vec![vec![0.0; terrain[0].len()]; terrain.len()];

    let kernel_size = 3;
    let sigma = 0.6;

    for y in 0..terrain.len() as i32 {
        for x in 0..terrain[0].len() as i32 {
            let mut sum = 0.0;
            let mut total_weight = 0.0;

            let center = Point {
                x: x as f64,
                y: y as f64,
            };

            for j in -kernel_size..kernel_size {
                for i in -kernel_size..kernel_size {
                    let p = Point {
                        x: center.x + i as f64,
                        y: center.y + j as f64,
                    };

                    let weight = gaussian_2d(p, center, sigma);

                    if let Some(elevation) = elevation_at(&terrain, p.x, p.y) {
                        sum += elevation * weight;
                        total_weight += weight;
                    }
                }
            }

            result[y as usize][x as usize] = sum / total_weight;
        }
    }

    result
}

fn perlin_terrain(width: usize, height: usize, seed: u32) -> Terrain {
    let hasher = PermutationTable::new(seed);

    let terrain = PlaneMapBuilder::new_fn(|mut point| {
        point[0] *= 2.0;
        point[1] *= 2.0;
        perlin_surflet_2d(point.into(), &hasher)
    })
    .set_size(width, height)
    .set_x_bounds(-10.0, 10.0)
    .set_y_bounds(-10.0, 10.0)
    .build();

    let mut result = blank_terrain(width, height);

    for y in 0..height {
        for x in 0..width {
            result[y][x] = terrain.get_value(x, y);
        }
    }

    return result;
}

fn island(width: usize, height: usize, center: Point, seed: u64, size: f64) -> Terrain {
    let mut rng = StdRng::seed_from_u64(seed);

    let mut terrain = blank_terrain(width, height);

    let perlin = perlin_terrain(width, height, rng.gen_range(0..1000));

    const PLATAEU: f64 = 0.5;

    for y in 0..height {
        for x in 0..width {
            let p = Point {
                x: x as f64,
                y: y as f64,
            };

            let weight = gaussian_2d(p, center, 4.) * 150.0 * size;

            terrain[y][x] = perlin[y][x].abs() * weight;

            if terrain[y][x] > PLATAEU {
                terrain[y][x] = PLATAEU;
            }
        }
    }

    terrain = gauss_blur(terrain);

    for y in 0..height {
        for x in 0..width {
            if terrain[y][x] < 0.0 {
                terrain[y][x] = 0.0;
            }
        }
    }

    return terrain;
}

fn generate_problem(seed: u64) -> Terrain {
    let (width, height) = (400, 400);

    let mut rng = StdRng::seed_from_u64(seed);

    let islands = 35;

    let mut terrain = blank_terrain(width, height);
    offset_terrain(&mut terrain, -0.1);

    for _ in 0..islands {
        let pos = Point {
            x: rng.gen_range(0..width) as f64,
            y: rng.gen_range(0..height) as f64,
        };

        let height = 50;
        let width = 50;
        let center = Point {
            x: width as f64 / 2.0,
            y: height as f64 / 2.0,
        };

        let t = island(width, height, center, rng.gen(), rng.gen_range(0.7..12.0));
        add_terrain(&mut terrain, &t, pos);

        if rng.gen_bool(0.2) {
            continue;
        }

        let mut lake = island(width, height, center, rng.gen(), rng.gen_range(0.05..0.2));
        scale_terrain(&mut lake, -3.0);
        add_terrain(&mut terrain, &lake, pos);

        if rng.gen_bool(0.2) {
            continue;
        }

        let mut recursive_island =
            island(width, height, center, rng.gen(), rng.gen_range(0.01..0.03));
        scale_terrain(&mut recursive_island, 6.0);
        add_terrain(&mut terrain, &recursive_island, pos);
    }

    let mut noise = perlin_terrain(width, height, 0);
    scale_terrain(&mut noise, 0.05);
    offset_terrain(&mut noise, 1.0);

    multiply_terrain(&mut terrain, &noise);

    terrain = gauss_blur(terrain);

    let recursive_islands = solve::recurive_islands(&terrain);
    if recursive_islands.len() < 3 {
        return generate_problem(seed + 1);
    }

    terrain
}
