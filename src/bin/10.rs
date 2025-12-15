advent_of_code::solution!(10);

#[derive(Debug)]
pub struct Machine {
    pub lights: Vec<bool>,
    pub buttons: Vec<Vec<usize>>,
    pub joltages: Vec<usize>,
}

impl TryFrom<&str> for Machine {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split_whitespace().collect();
        if parts.len() < 3 {
            return Err("Invalid input format: too few parts");
        }

        // Parse lights
        let lights_str = parts[0];
        if !lights_str.starts_with('[') || !lights_str.ends_with(']') {
            return Err("Invalid lights format");
        }
        let lights: Vec<bool> = lights_str[1..lights_str.len() - 1]
            .chars()
            .map(|c| match c {
                '.' => Ok(false),
                '#' => Ok(true),
                _ => Err("Invalid light character"),
            })
            .collect::<Result<_, _>>()?;

        // Parse joltages (last part)
        let joltages_str = parts[parts.len() - 1];
        if !joltages_str.starts_with('{') || !joltages_str.ends_with('}') {
            return Err("Invalid joltages format");
        }
        let joltages: Vec<usize> = joltages_str[1..joltages_str.len() - 1]
            .split(',')
            .map(|s| s.parse::<usize>().map_err(|_| "Invalid joltage number"))
            .collect::<Result<_, _>>()?;

        // Parse buttons (middle parts)
        let mut buttons = Vec::new();
        for button_str in &parts[1..parts.len() - 1] {
            if !button_str.starts_with('(') || !button_str.ends_with(')') {
                return Err("Invalid button format");
            }
            let button: Vec<usize> = button_str[1..button_str.len() - 1]
                .split(',')
                .map(|s| s.parse::<usize>().map_err(|_| "Invalid button number"))
                .collect::<Result<_, _>>()?;
            buttons.push(button);
        }

        Ok(Machine {
            lights,
            buttons,
            joltages,
        })
    }
}

impl Machine {
    pub fn fewest_buttons(&self) -> u64 {
        let n_equations = self.lights.len();
        let n_vars = self.buttons.len();

        // Build augmented matrix: [A | b]
        // A[i][j] = 1 if button j affects light i
        // b[i] = 1 if light i needs to be ON (true)
        // Storing as Vec<Vec<u8>> for simplicity, optimizing to bitsets if needed later.
        // Using u8 for GF(2) elements (0 or 1).
        let mut mat = vec![vec![0u8; n_vars + 1]; n_equations];

        for (j, button) in self.buttons.iter().enumerate() {
            for &light_idx in button {
                if light_idx < n_equations {
                    mat[light_idx][j] = 1;
                }
            }
        }

        for (i, &light_on) in self.lights.iter().enumerate() {
            if light_on {
                mat[i][n_vars] = 1;
            }
        }

        // Gaussian Elimination to RREF
        let mut pivot_row = 0;
        let mut pivot_cols = vec![None; n_equations]; // Maps row -> col (pivot variable)
        let mut free_vars = Vec::new();

        for col in 0..n_vars {
            if pivot_row >= n_equations {
                free_vars.push(col);
                continue;
            }

            // Find pivot
            let mut pivot = None;
            for row in pivot_row..n_equations {
                if mat[row][col] == 1 {
                    pivot = Some(row);
                    break;
                }
            }

            match pivot {
                Some(row) => {
                    // Swap rows
                    mat.swap(pivot_row, row);

                    // Eliminate other rows
                    for r in 0..n_equations {
                        if r != pivot_row && mat[r][col] == 1 {
                            // Row operation: r = r XOR pivot_row
                            for k in col..=n_vars {
                                mat[r][k] ^= mat[pivot_row][k];
                            }
                        }
                    }

                    pivot_cols[pivot_row] = Some(col);
                    pivot_row += 1;
                }
                None => {
                    free_vars.push(col);
                }
            }
        }

        // Check for inconsistency
        // If a row is all zeros in A part but 1 in b, then 0 = 1 => impossible
        for row in pivot_row..n_equations {
            if mat[row][n_vars] == 1 {
                // Inconsistent system
                // Problem statement implies a solution exists ("What is the fewest...").
                // But let's return a large number or 0 if we assume valid input.
                // Given the phrasing, valid config is expected.
                // For robustness, panic or return max.
                panic!("No solution found for machine");
            }
        }

        // Try all combinations of free variables
        let n_free = free_vars.len();
        let mut min_presses = u64::MAX;

        for i in 0..(1 << n_free) {
            let mut solution = vec![0u8; n_vars];
            let mut presses = 0;

            // Set free variables
            for (bit, &var_idx) in free_vars.iter().enumerate() {
                if (i >> bit) & 1 == 1 {
                    solution[var_idx] = 1;
                    presses += 1;
                }
            }

            // Solve for pivot variables (Back substitution effectively done by RREF)
            // For each pivot row, x_pivot = b_row XOR sum(x_free * A_row_free)
            // Since we zeroed out everything above/below pivot in that col,
            // the equation is just: x_pivot + sum(existing 1s to the right) = b
            // So x_pivot = b XOR sum(...)

            for row in 0..pivot_row {
                if let Some(pivot_col) = pivot_cols[row] {
                    let mut val = mat[row][n_vars];
                    for col in (pivot_col + 1)..n_vars {
                        if mat[row][col] == 1 {
                            val ^= solution[col];
                        }
                    }
                    solution[pivot_col] = val;
                    if val == 1 {
                        presses += 1;
                    }
                }
            }

            if presses < min_presses {
                min_presses = presses;
            }
        }

        min_presses
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let res = input
        .lines()
        .map(|line| Machine::try_from(line).unwrap())
        .map(|machine| machine.fewest_buttons())
        .sum();
    Some(res)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let machine = Machine::try_from(input).unwrap();

        assert_eq!(machine.lights, vec![false, true, true, false]);
        assert_eq!(
            machine.buttons,
            vec![
                vec![3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![0, 2],
                vec![0, 1]
            ]
        );
        assert_eq!(machine.joltages, vec![3, 5, 4, 7]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
