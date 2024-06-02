use std::collections::HashSet;

use crate::Terrain;

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
    
    fn edge(&self, terrain: &Terrain) -> Option<(i64, i64)> {
        let (mut x, y) = self.points[0];

        while terrain[y as usize][x as usize] > 0.0 {
            x += 1;

            if x < 0 || x >= terrain[0].len() as i64 {
                return None;
            }
        }

        assert!(self.connected(x, y));

        Some((x, y))
    }
}

fn find_land_masses(terrain: &Terrain) -> Vec<LandMass> {
    let mut land_masses: Vec<LandMass> = vec![];

    for x in 0..terrain[0].len() {
        for y in 0..terrain.len() {
            if terrain[y][x] > 0.0 {
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

pub fn recurive_islands(terrain: &Terrain) -> Vec<LandMass> {
    let land_masses = find_land_masses(terrain);

    let mut recursive_islands: Vec<LandMass> = vec![];

    for (i, inner_land_mass) in land_masses.iter().enumerate() {
        for (j, outer_land_mass) in land_masses.iter().enumerate() {
            if i == j {
                continue;
            }

            if !outer_land_mass.contains_land_mass(inner_land_mass) {
                continue;
            }

            let edge = inner_land_mass.edge(terrain);

            if edge.is_none() {
                continue;
            }

            let (x, y) = edge.unwrap();

            if x < 0 || y < 0 || x >= terrain[0].len() as i64 || y >= terrain.len() as i64 {
                continue;
            }
            
            let surrounded = lake_flood(terrain, x, y, outer_land_mass);

            if surrounded {
                recursive_islands.push(inner_land_mass.clone());
            }
        }
    }

    recursive_islands
}

fn lake_flood(terrain: &Terrain, x: i64, y: i64, bounding_box: &LandMass) -> bool {
    let mut visited = HashSet::new();
    __lake_flood(terrain, x, y, &mut visited, bounding_box)
}

fn __lake_flood(
    terrain: &Terrain,
    x: i64,
    y: i64,
    visited: &mut HashSet<(i64, i64)>,
    bounding_box: &LandMass,
) -> bool {
    if x < 0 || y < 0 || x >= terrain[0].len() as i64 || y >= terrain.len() as i64 {
        return false;
    }

    if terrain[y as usize][x as usize] > 0.0 || visited.contains(&(x, y)) {
        return true;
    }

    visited.insert((x, y));

    if !bounding_box.within_bounding_box(x, y) {
        return false;
    }

    __lake_flood(terrain, x + 1, y, visited, bounding_box)
        && __lake_flood(terrain, x - 1, y, visited, bounding_box)
        && __lake_flood(terrain, x, y + 1, visited, bounding_box)
        && __lake_flood(terrain, x, y - 1, visited, bounding_box)
}
