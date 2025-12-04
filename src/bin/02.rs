advent_of_code::solution!(2);

#[derive(Debug)]
pub struct IDRange {
    start: u64,
    end: u64,
}

impl IDRange {
    pub fn find_invalid(&self) -> Vec<u64> {
        let mut id = self.start;
        let mut invalid_ids = Vec::new();

        while id <= self.end {
            let s = id.to_string();
            let len = s.len();
            if len % 2 == 1 {
                let x = 10_u64.pow(len as u32 / 2);
                id = x * x * 10 + x;
                continue;
            }
            let x = 10_u64.pow(len as u32 / 2);
            let upper = id / x;
            let lower = id % x;
            if upper == lower {
                invalid_ids.push(id);
                id = (upper + 1) * x + (upper + 1);
            } else if upper < lower {
                id = (upper + 1) * x + upper + 1;
            } else {
                id = upper * x + upper;
            }
        }

        invalid_ids
    }

    pub fn find_invalid_v2(&self) -> Vec<u64> {
        let mut invalid_ids = Vec::new();
        for id in self.start..=self.end {
            let s = id.to_string();
            let len = s.len();
            if len < 2 {
                continue;
            }

            for l in 1..=len / 2 {
                if len % l == 0 {
                    let pattern = &s[0..l];
                    if pattern.repeat(len / l) == s {
                        invalid_ids.push(id);
                        break;
                    }
                }
            }
        }
        invalid_ids
    }
}

impl TryFrom<&str> for IDRange {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (start, end) = value
            .trim()
            .split_once('-')
            .ok_or("Invalid range format, expected 'start-end'")?;
        let start: u64 = start.parse().map_err(|_| "Invalid start ID")?;
        let end: u64 = end.parse().map_err(|_| "Invalid end ID")?;
        Ok(Self { start, end })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let res = input
        .split(",")
        .map(|s| IDRange::try_from(s).expect("invalid input"))
        .map(|r| r.find_invalid())
        .flatten()
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let res = input
        .split(",")
        .map(|s| IDRange::try_from(s).expect("invalid input"))
        .map(|r| r.find_invalid_v2())
        .flatten()
        .sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
