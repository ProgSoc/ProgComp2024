#![allow(dead_code)]
#![allow(unused_imports)]

use std::collections::HashSet;

use crate::{print_terrain, Terrain};


type LandTerrain = Vec<Vec<bool>>;
static WATER: bool = false;
static LAND: bool = true;

/// Returns None if there was not enough recursive islands.
pub fn solve(terrain: &Terrain) -> Option<i32> {
    let mut land: LandTerrain = terrain
        .iter()
        .map(|row| row.iter().map(|&x| x > 0.0).collect())
        .collect();

    let x: i32 = 0;
    let y: i32 = 0;

    assert_eq!(land[x as usize][y as usize], WATER);

    flood(&mut land, x, y, &LAND);
    flood(&mut land, x, y, &WATER);

    let land_masses = find_land_masses(&land);

    if land_masses.len() < 3 {
        return None;
    }

    return Some(
        land.iter()
            .map(|row| row.iter().filter(|&&x| x).count())
            .sum::<usize>() as i32,
    );
}

fn flood(land_terrain: &mut LandTerrain, x: i32, y: i32, fill_with: &bool) {
    let mut stack: Vec<(i32, i32)> = vec![(x, y)];

    while let Some((x, y)) = stack.pop() {
        if x < 0 || x >= land_terrain[0].len() as i32 {
            continue;
        }

        if y < 0 || y >= land_terrain.len() as i32 {
            continue;
        }

        if land_terrain[y as usize][x as usize] == *fill_with {
            continue;
        }

        land_terrain[y as usize][x as usize] = *fill_with;

        stack.push((x + 1, y));
        stack.push((x - 1, y));
        stack.push((x, y + 1));
        stack.push((x, y - 1));
    }
}


/// Used to count the number of recursive islands to ensure that there are at least 3
/// when generating the question.
#[derive(Clone)]
pub struct LandMass {
    points: Vec<(i64, i64)>,
    greatest_x: i64,
    greatest_y: i64,
    smallest_x: i64,
    smallest_y: i64,
}

impl LandMass {
    pub fn area(&self) -> usize {
        self.points.len()
    }

    fn within_bounding_box(&self, x: i64, y: i64) -> bool {
        x >= self.smallest_x - 1
            && x <= self.greatest_x + 1
            && y >= self.smallest_y - 1
            && y <= self.greatest_y + 1
    }

    fn connected(&self, x: i64, y: i64) -> bool {
        if !self.within_bounding_box(x, y) {
            return false;
        }

        for (dx, dy) in &self.points {
            if (x - dx).abs() <= 1 && (y - dy).abs() <= 1 {
                return true;
            }
        }

        false
    }

    fn add(&mut self, x: i64, y: i64) {
        self.points.push((x, y));

        if x > self.greatest_x {
            self.greatest_x = x;
        }

        if y > self.greatest_y {
            self.greatest_y = y;
        }

        if x < self.smallest_x {
            self.smallest_x = x;
        }

        if y < self.smallest_y {
            self.smallest_y = y;
        }
    }

    fn new(x: i64, y: i64) -> LandMass {
        LandMass {
            points: vec![(x, y)],
            greatest_x: x,
            greatest_y: y,
            smallest_x: x,
            smallest_y: y,
        }
    }

    fn coalesce(mut self, other: LandMass) -> LandMass {
        for (x, y) in &other.points {
            self.add(*x, *y);
        }

        self
    }

    fn contains_land_mass(&self, other: &LandMass) -> bool {
        self.greatest_x >= other.greatest_x
            && self.greatest_y >= other.greatest_y
            && self.smallest_x <= other.smallest_x
            && self.smallest_y <= other.smallest_y
    }
}

fn find_land_masses(terrain: &LandTerrain) -> Vec<LandMass> {
    let mut land_masses: Vec<LandMass> = vec![];

    for x in 0..terrain[0].len() {
        for y in 0..terrain.len() {
            if terrain[y][x] == LAND {
                let x = x as i64;
                let y = y as i64;

                let mut connected_to: Vec<usize> = vec![];

                for (i, land_mass) in land_masses.iter().enumerate() {
                    if land_mass.connected(x, y) {
                        connected_to.push(i);
                    }
                }

                if connected_to.len() == 0 {
                    land_masses.push(LandMass::new(x, y));
                } else if connected_to.len() == 1 {
                    land_masses[connected_to[0]].add(x, y);
                } else {
                    connected_to.reverse();

                    let mut coalesced = land_masses.remove(connected_to[0]);

                    for i in connected_to.iter().skip(1) {
                        coalesced = coalesced.coalesce(land_masses.remove(*i));
                    }

                    coalesced.add(x, y);

                    land_masses.push(coalesced);
                }
            }
        }
    }

    land_masses
}
