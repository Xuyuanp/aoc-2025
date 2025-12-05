advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let total_joltage = input
        .lines()
        .map(|line| {
            // We need to find the maximum 2-digit number we can form from the line.
            // A brute-force check of all pairs (i, j) where i < j is efficient enough.
            let mut max_line_joltage = 0;
            let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

            if digits.len() < 2 {
                return 0; // Or handle as an error, but 0 doesn't affect the sum.
            }

            for i in 0..digits.len() {
                for j in (i + 1)..digits.len() {
                    let current_joltage = digits[i] * 10 + digits[j];
                    if current_joltage > max_line_joltage {
                        max_line_joltage = current_joltage;
                    }
                }
            }
            max_line_joltage as u64
        })
        .sum();

    Some(total_joltage)
}

pub fn part_two(input: &str) -> Option<u64> {
    let res: u64 = input
        .lines()
        .map(|line| {
            let digits: Vec<u8> = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect();
            let n = digits.len();
            let k = 12;

            let mut result_digits = Vec::with_capacity(k);
            let mut start_pos = 0;

            for i in 0..k {
                let remaining = k - i;
                let end_pos = n - remaining;

                // The window is digits[start_pos..=end_pos]
                let window = &digits[start_pos..=end_pos];

                let mut best_digit = 0;
                let mut best_digit_pos_in_window = 0;

                // Find the first occurrence of the highest possible digit.
                for (pos, &digit) in window.iter().enumerate() {
                    if digit > best_digit {
                        best_digit = digit;
                        best_digit_pos_in_window = pos;
                        if best_digit == 9 {
                            break; // Found the best possible, can stop.
                        }
                    }
                }

                result_digits.push(best_digit);
                // Next search starts after the digit we just picked.
                start_pos += best_digit_pos_in_window + 1;
            }

            result_digits
                .iter()
                .fold(0, |acc, &digit| acc * 10 + digit as u64)
        })
        .sum();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
