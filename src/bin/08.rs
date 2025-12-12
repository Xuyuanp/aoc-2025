use std::collections::HashMap;

advent_of_code::solution!(8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position3D {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

impl Position3D {
    pub fn distance(&self, other: &Position3D) -> f64 {
        let dx = self.x.abs_diff(other.x) as f64;
        let dy = self.y.abs_diff(other.y) as f64;
        let dz = self.z.abs_diff(other.z) as f64;

        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

impl TryFrom<&str> for Position3D {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let coords: Vec<&str> = s.split(',').collect();
        if coords.len() != 3 {
            return Err("Invalid position format");
        }
        let x = coords[0]
            .parse::<u64>()
            .map_err(|_| "Invalid x coordinate")?;
        let y = coords[1]
            .parse::<u64>()
            .map_err(|_| "Invalid y coordinate")?;
        let z = coords[2]
            .parse::<u64>()
            .map_err(|_| "Invalid z coordinate")?;
        Ok(Position3D { x, y, z })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Pair(Position3D, Position3D);

impl Pair {
    pub fn distance(&self) -> f64 {
        self.0.distance(&self.1)
    }
}

#[cfg(not(test))]
const CONNECTIONS: usize = 1000;

#[cfg(test)]
const CONNECTIONS: usize = 10;

pub struct Solution {
    pub positions: Vec<Position3D>,
    pub pairs: Vec<Pair>,
    pub circuits: HashMap<usize, Vec<Position3D>>,
    pub indexer: HashMap<Position3D, usize>,
}

impl TryFrom<&str> for Solution {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let positions: Vec<Position3D> = s
            .lines()
            .map(|line| Position3D::try_from(line))
            .collect::<Result<_, _>>()?;

        let mut pairs: Vec<Pair> = positions
            .iter()
            .enumerate()
            .flat_map(|(i, pos1)| {
                positions
                    .iter()
                    .skip(i + 1)
                    .map(move |pos2| Pair(*pos1, *pos2))
            })
            .collect();

        pairs.sort_by(|a, b| a.distance().partial_cmp(&b.distance()).unwrap());

        let mut circuits: HashMap<usize, Vec<Position3D>> = HashMap::new();
        let mut indexer: HashMap<Position3D, usize> = HashMap::new();
        positions.iter().enumerate().for_each(|(i, pos)| {
            circuits.insert(i, vec![*pos]);
            indexer.insert(*pos, i);
        });

        Ok(Solution {
            positions,
            pairs,
            circuits,
            indexer,
        })
    }
}

impl Solution {
    pub fn part_one(&mut self) -> u64 {
        for pair in self.pairs.iter().take(CONNECTIONS) {
            let c1 = self.indexer.get(&pair.0).unwrap().clone();
            let c2 = self.indexer.get(&pair.1).unwrap().clone();
            if c1 == c2 {
                continue;
            }
            let postions = self.circuits.remove(&c2).unwrap();
            self.circuits.get_mut(&c1).unwrap().extend(postions.iter());
            for pos in postions {
                self.indexer.insert(pos, c1);
            }
        }

        let mut lens: Vec<u64> = self.circuits.values().map(|c| c.len() as u64).collect();
        lens.sort();

        lens.iter().rev().take(3).product()
    }

    pub fn part_two(&mut self) -> u64 {
        for pair in self.pairs.iter() {
            let c1 = self.indexer.get(&pair.0).unwrap().clone();
            let c2 = self.indexer.get(&pair.1).unwrap().clone();
            if c1 == c2 {
                continue;
            }
            let postions = self.circuits.remove(&c2).unwrap();
            self.circuits.get_mut(&c1).unwrap().extend(postions.iter());
            for pos in postions {
                self.indexer.insert(pos, c1);
            }

            if self.circuits.len() == 1 {
                return pair.0.x * pair.1.x;
            }
        }

        unreachable!()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sol = Solution::try_from(input).expect("invalid input");
    Some(sol.part_one())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sol = Solution::try_from(input).expect("invalid input");
    Some(sol.part_two())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
