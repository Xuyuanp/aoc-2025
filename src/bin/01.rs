advent_of_code::solution!(1);

#[derive(Debug)]
pub enum Rotation {
    L(u64),
    R(u64),
}

impl TryFrom<&str> for Rotation {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (dir, dist) = value.split_at(1);
        let dist: u64 = dist.parse().map_err(|_| "Invalid distance")?;
        match dir {
            "L" => Ok(Rotation::L(dist)),
            "R" => Ok(Rotation::R(dist)),
            _ => Err("Invalid direction"),
        }
    }
}

const MAX_DISTANCE: u64 = 100;

impl Rotation {
    pub fn rotate(&self, start: u64) -> u64 {
        match self {
            Rotation::L(dist) => (start + MAX_DISTANCE - dist % MAX_DISTANCE) % MAX_DISTANCE,
            Rotation::R(dist) => (start + dist) % MAX_DISTANCE,
        }
    }

    pub fn rotate_v2(&self, start: u64) -> (u64, u64) {
        match self {
            Rotation::L(dist) => {
                let times = dist / MAX_DISTANCE;
                let dist = dist % MAX_DISTANCE;

                if start == 0 {
                    return (MAX_DISTANCE - dist, times);
                }

                let Some(end) = start.checked_sub(dist) else {
                    return (start + MAX_DISTANCE - dist, times + 1);
                };
                (end, times + (end == 0) as u64)
            }
            Rotation::R(dist) => {
                let end = start + dist;
                let times = end / MAX_DISTANCE;
                let end = end % MAX_DISTANCE;
                (end, times)
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, res) = input
        .lines()
        .map(|line| Rotation::try_from(line).expect("input should be valid"))
        .fold((50_u64, 0_u64), |(curr, res), rotation| {
            let next_curr = rotation.rotate(curr);
            (next_curr, res + (next_curr == 0) as u64)
        });
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, res) = input
        .lines()
        .map(|line| Rotation::try_from(line).expect("input should be valid"))
        .fold((50_u64, 0_u64), |(curr, res), rotation| {
            let (next_curr, times) = rotation.rotate_v2(curr);
            (next_curr, res + times)
        });
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
