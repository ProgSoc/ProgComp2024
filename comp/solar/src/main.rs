use std::{
    collections::hash_map::DefaultHasher,
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

    let mut rng = StdRng::seed_from_u64(seed);

    let alpha = rng.gen_range(1.0..5.0);

    match args[1].as_str() {
        "generate" => {
            println!("{}", alpha);
        }
        "validate" => {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();

            let input = buffer
                .trim()
                .parse::<f64>()
                .expect("Invalid input. Expected number.");

            let solution: f64 = simulate(alpha);

            if (input - solution).abs() < 0.5 {
                exit(0);
            } else {
                exit(1);
            }
        }
        _ => panic!(),
    }
}

struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn mag(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    fn angle_between(&self, other: &Vec3) -> f64 {
        self.dot(other) / (self.mag() * other.mag())
    }
}

struct Angle {
    lat: f64,
    long: f64,
}

impl Angle {
    fn to_vec3(&self) -> Vec3 {
        Vec3 {
            x: self.lat.cos() * self.long.cos(),
            y: self.lat.cos() * self.long.sin(),
            z: self.lat.sin(),
        }
    }

    fn angle_between(&self, other: &Angle) -> f64 {
        self.to_vec3().angle_between(&other.to_vec3())
    }
}

const SUN_LAT: f64 = -23.5_f64 * (std::f64::consts::PI / 180.0);
const STATION_LAT: f64 = -76.0_f64 * (std::f64::consts::PI / 180.0);

/// Power output in watts
fn power(alpha: f64, time: f64, tilt: f64) -> f64 {
    assert!(tilt.abs() <= 15.0_f64.to_radians());

    let pointing = Angle {
        lat: STATION_LAT + tilt,
        long: long_of_time(time),
    };

    let sun = Angle {
        lat: SUN_LAT,
        long: 0.0,
    };

    let angle = sun.angle_between(&pointing);

    if angle < 0.0 {
        return 0.0;
    }

    alpha * angle
}

const SECONDS_IN_DAY: f64 = 86400.0;

fn long_of_time(time: f64) -> f64 {
    (time / SECONDS_IN_DAY) * 2.0 * std::f64::consts::PI
}

/// Optimal tilt in radians
/// Does not account for power lost to motor just finds angle that maximises power output
fn optimise_tilt(alpha: f64, time: f64) -> f64 {
    let mut best_tilt = 0.0;
    let mut best_power = 0.0;

    for tilt in (-100..100).map(|x| x as f64) {
        let tilt = tilt as f64 / 100. * 15.0_f64.to_radians();

        let power = power(alpha, time, tilt);

        if power > best_power {
            best_power = power;
            best_tilt = tilt;
        }
    }

    best_tilt
}

fn simulate(alpha: f64) -> f64 {
    let mut total_energy = 0.0;

    let mut previous_motor_tilt: f64 = 0.0;

    for time in 0..SECONDS_IN_DAY as i32 {
        let time = time as f64;

        let motor_tilt: f64 = optimise_tilt(alpha, time);

        total_energy += power(alpha, time, motor_tilt);
        total_energy -= (motor_tilt - previous_motor_tilt).abs();

        previous_motor_tilt = motor_tilt;
    }

    total_energy
}
