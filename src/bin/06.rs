advent_of_code::solution!(6);

pub enum Op {
    Add,
    Mul,
}

impl TryFrom<&str> for Op {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(Op::Add),
            "*" => Ok(Op::Mul),
            _ => Err(()),
        }
    }
}

impl Op {
    pub fn mempty(&self) -> u64 {
        match self {
            Op::Add => 0,
            Op::Mul => 1,
        }
    }

    pub fn mappend(&self, a: u64, b: u64) -> u64 {
        match self {
            Op::Add => a + b,
            Op::Mul => a * b,
        }
    }
}

pub fn transpose<T: Default + Clone + Copy>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    if matrix.is_empty() {
        return vec![];
    }

    let max_len = matrix.iter().map(|row| row.len()).max().unwrap_or(0);
    if max_len == 0 {
        return vec![];
    }

    let new_rows = max_len;
    let new_cols = matrix.len();

    let mut transposed = vec![vec![T::default(); new_cols]; new_rows];

    for (i, row) in matrix.iter().enumerate() {
        for (j, &item) in row.iter().enumerate() {
            if j < new_rows {
                transposed[j][i] = item;
            }
        }
    }

    transposed
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines().rev();
    let ops: Vec<Op> = lines
        .next()
        .expect("ops line not exists")
        .split_whitespace()
        .map(|op| Op::try_from(op).expect("invalid op"))
        .collect();
    let numbers = lines
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u64>().expect("invalid number"))
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let res = ops
        .iter()
        .enumerate()
        .map(|(i, op)| {
            numbers
                .iter()
                .map(|nums| nums[i])
                .fold(op.mempty(), |acc, x| op.mappend(acc, x))
        })
        .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines().rev();
    let mut ops = lines
        .next()
        .expect("ops line not exists")
        .split_whitespace()
        .map(|op| Op::try_from(op).expect("invalid op"));

    let numbers = lines
        .map(|line| line.as_bytes().iter().map(|b| *b).collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();
    let transposed_numbers = transpose(&numbers);
    let columns: Vec<String> = transposed_numbers
        .iter()
        .map(|line| {
            let mut line = line.clone();
            line.reverse();
            String::from_utf8(line).expect("invalid input")
        })
        .collect();

    let mut res = 0;
    let mut curr_op = ops.next().expect("no op found");
    let mut curr_val = curr_op.mempty();
    for col in columns {
        let col = col.trim();
        if col.is_empty() {
            res += curr_val;
            curr_op = ops.next().expect("no op found");
            curr_val = curr_op.mempty();
            continue;
        }
        curr_val = curr_op.mappend(
            curr_val,
            col.parse::<u64>().expect("invalid number in part two"),
        );
    }
    res += curr_val;

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
