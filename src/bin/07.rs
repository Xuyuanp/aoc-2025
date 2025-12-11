use std::collections::{HashMap, HashSet};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut res = 0;

    let mut lines = input.lines();
    let start = lines.next()?.find(|b| b == 'S')?;
    let mut beams: HashSet<_> = [start].into();

    for line in lines {
        let mut next_beams = HashSet::new();
        for i in line
            .chars()
            .enumerate()
            .filter(|(_, b)| *b == '^')
            .map(|(i, _)| i)
        {
            if !beams.remove(&i) {
                continue;
            }
            if i > 0 {
                next_beams.insert(i - 1);
            }
            if i + 1 < line.len() {
                next_beams.insert(i + 1);
            }
            res += 1;
        }
        beams.extend(next_beams);
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut memo: HashMap<(usize, usize), u64> = HashMap::new();

    let lines: Vec<&str> = input.lines().collect();
    let start_col = lines[0].find(|b| b == 'S')?;

    fn dfs(
        row: usize,
        col: usize,
        memo: &mut HashMap<(usize, usize), u64>,
        lines: &Vec<&str>,
    ) -> u64 {
        if let Some(&res) = memo.get(&(row, col)) {
            return res;
        }
        let res = {
            if row == lines.len() - 1 {
                1
            } else if lines[row].chars().nth(col).unwrap() != '^' {
                dfs(row + 1, col, memo, lines)
            } else {
                let mut res = 0;
                if col > 0 {
                    res += dfs(row + 1, col - 1, memo, lines);
                }
                if col + 1 < lines[row].len() {
                    res += dfs(row + 1, col + 1, memo, lines);
                }
                res
            }
        };
        memo.insert((row, col), res);
        res
    }

    let res = dfs(1, start_col, &mut memo, &lines);
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
