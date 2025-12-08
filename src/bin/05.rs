use std::ops::Range;

advent_of_code::solution!(5);

#[derive(Debug)]
pub struct DB {
    ranges: Vec<Range<u64>>,
    ids: Vec<u64>,
}

impl TryFrom<&str> for DB {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let ranges = value
            .lines()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                let (left, right) = line.split_once("-").expect("invalid range");
                let l: u64 = left.parse().expect("invalid number");
                let r: u64 = right.parse().expect("invalid number");
                Range {
                    start: l,
                    end: r + 1,
                }
            })
            .collect::<Vec<Range<u64>>>();

        let ids = value
            .lines()
            .skip_while(|line| !line.is_empty())
            .skip(1)
            .map(|line| line.parse().expect("invalid id"))
            .collect::<Vec<u64>>();
        Ok(Self { ranges, ids })
    }
}

impl DB {
    pub fn solution(&self) -> u64 {
        self.ids
            .iter()
            .filter(|id| self.ranges.iter().any(|r| r.contains(id)))
            .count() as u64
    }

    pub fn solution_v2(&mut self) -> u64 {
        self.ranges.sort_by(|r1, r2| r1.start.cmp(&r2.start));
        let mut merged_ranges = vec![self.ranges[0].clone()];
        for r in self.ranges.iter().skip(1) {
            let last = merged_ranges.last_mut().unwrap();
            if r.start <= last.end {
                last.end = last.end.max(r.end);
            } else {
                merged_ranges.push(r.clone());
            }
        }

        merged_ranges.iter().map(|r| r.end - r.start).sum()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let db = DB::try_from(input).expect("invalid input");
    Some(db.solution())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut db = DB::try_from(input).expect("invalid input");
    Some(db.solution_v2())
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
        assert_eq!(result, Some(14));
    }
}
