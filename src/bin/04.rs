advent_of_code::solution!(4);

const PAPER: u8 = b'@';
const REMOVED: u8 = b'x';

pub struct Grid {
    pub data: [[u8; 136]; 136],
    pub m: usize,
    pub n: usize,
}

impl Grid {
    pub fn new(data: [[u8; 136]; 136], m: usize, n: usize) -> Self {
        Self { data, m, n }
    }

    pub fn neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        for dr in [-1isize, 0, 1].iter() {
            for dc in [-1isize, 0, 1].iter() {
                if *dr == 0 && *dc == 0 {
                    continue;
                }
                let new_row = row as isize + dr;
                let new_col = col as isize + dc;

                if new_row >= 0
                    && new_row < self.m as isize
                    && new_col >= 0
                    && new_col < self.n as isize
                {
                    result.push((new_row as usize, new_col as usize));
                }
            }
        }

        result
    }

    pub fn is_paper(&self, row: usize, col: usize) -> bool {
        self.data[row][col] == PAPER
    }

    pub fn accessable(&self, row: usize, col: usize) -> bool {
        if !self.is_paper(row, col) {
            return false;
        }
        let neighbors = self.neighbors(row, col);
        neighbors
            .iter()
            .filter(|(r, c)| self.is_paper(*r, *c))
            .count()
            < 4
    }

    pub fn solution(&self) -> u64 {
        let mut count = 0u64;

        for i in 0..self.m {
            for j in 0..self.n {
                if self.accessable(i, j) {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn solution_v2(&mut self) -> u64 {
        let mut count = 0u64;

        for i in 0..self.m {
            for j in 0..self.n {
                if self.accessable(i, j) {
                    count += 1;
                    self.data[i][j] = REMOVED;
                }
            }
        }
        if count > 0 {
            count += self.solution_v2();
        }

        count
    }
}

impl TryFrom<&str> for Grid {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut data = [[0u8; 136]; 136];
        let mut m = 0;
        let mut n = 0;

        for (i, line) in value.lines().enumerate() {
            m = i + 1;
            let bytes = line.as_bytes();
            n = bytes.len();
            for (j, &b) in bytes.iter().enumerate() {
                data[i][j] = b;
            }
        }

        Ok(Self::new(data, m, n))
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::try_from(input).expect("Failed to parse grid");

    Some(grid.solution())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = Grid::try_from(input).expect("Failed to parse grid");

    Some(grid.solution_v2())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
