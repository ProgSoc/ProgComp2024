use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    ops::{Add, Mul, Sub},
    process::exit,
};

use rand::{Rng, SeedableRng};

type Canvas = Vec<Vec<bool>>;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    assert_eq!(args.len(), 3);

    let mut s = DefaultHasher::new();
    args[2].hash(&mut s);
    let seed = s.finish();

    let mut canvas = create_canvas(100, 100);
    paint_crude_circle(&mut canvas, point(50, 50), 35, &Brush { radius: 1 }, seed);

    match args[1].as_str() {
        "generate" => {
            print_canvas(&canvas);
        }
        "validate" => {
            let mut count = 0;
            assert!(!canvas[50][50]);
            flood(&mut canvas, 50, 50, &mut count);

            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();

            let input = buffer
                .trim()
                .parse::<u32>()
                .expect("Invalid input. Expected positive integer");

            if count == input {
                exit(0);
            } else {
                exit(1);
            }
        }
        _ => panic!(),
    }
}

fn create_canvas(width: i32, height: i32) -> Canvas {
    vec![vec![false; width as usize]; height as usize]
}

struct Brush {
    radius: i32,
}

fn inside_canvas(canvas: &Canvas, x: i32, y: i32) -> bool {
    x >= 0 && y >= 0 && x < canvas[0].len() as i32 && y < canvas.len() as i32
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Point {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

fn point(x: i32, y: i32) -> Point {
    Point { x, y }
}

fn dist(p1: Point, p2: Point) -> f64 {
    let x = p1.x as f64 - p2.x as f64;
    let y = p1.y as f64 - p2.y as f64;
    (x * x + y * y).sqrt()
}

fn draw_brush(canvas: &mut Canvas, p: Point, brush: &Brush) {
    for x in p.x - brush.radius..=p.x + brush.radius {
        for y in p.y - brush.radius..=p.y + brush.radius {
            if inside_canvas(canvas, x, y) && dist(p, point(x, y)) <= brush.radius as f64 {
                canvas[y as usize][x as usize] = true;
            }
        }
    }
}

fn paint_line(canvas: &mut Canvas, start: Point, end: Point, brush: &Brush) {
    let steps = (dist(start, end) + 1.) as i32;

    for i in 0..steps {
        let dif = end - start;

        let point_in_line = i as f64 / steps as f64;

        let x_ = dif.x as f64 * point_in_line;
        let y_ = dif.y as f64 * point_in_line;

        let p = start + point(x_ as i32, y_ as i32);
        draw_brush(canvas, p, brush);
    }
}

fn paint_crude_circle(canvas: &mut Canvas, center: Point, radius: i32, brush: &Brush, seed: u64) {
    let start = center + point(radius, 0);

    let (mut brush_x, mut brush_y) = (start.x as f64, start.y as f64);

    const POINTS: i32 = 40;
    const STABISATION: f64 = 0.6;
    const WIGGLE: i32 = 15;

    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    for theta in 0..POINTS {
        let theta = (theta as f64 / POINTS as f64) * 2. * std::f64::consts::PI;
        let from = point(brush_x as i32, brush_y as i32);

        let radius = (radius + rng.gen::<i32>() % WIGGLE - (WIGGLE / 2)) as f64;

        let x = center.x + (radius * theta.cos()) as i32;
        let y = center.y + (radius * theta.sin()) as i32;

        let target = point(x, y);

        brush_x = lerp(brush_x, target.x as f64, 1.0 - STABISATION);
        brush_y = lerp(brush_y, target.y as f64, 1.0 - STABISATION);

        let to = point(brush_x as i32, brush_y as i32);
        paint_line(canvas, from, to, brush);
    }

    paint_line(canvas, point(brush_x as i32, brush_y as i32), start, brush);
}

fn print_canvas(canvas: &Canvas) {
    for row in canvas {
        println!(
            "{}",
            row.iter()
                .map(|&b| if b { "1" } else { "0" })
                .collect::<Vec<&str>>()
                .join(", ")
        );
    }
}

fn flood(canvas: &mut Canvas, x: i32, y: i32, count: &mut u32) {
    if !inside_canvas(canvas, x, y) || canvas[y as usize][x as usize] {
        return;
    }

    canvas[y as usize][x as usize] = true;

    *count += 1;

    flood(canvas, x + 1, y, count);
    flood(canvas, x - 1, y, count);
    flood(canvas, x, y + 1, count);
    flood(canvas, x, y - 1, count);
}
