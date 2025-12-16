advent_of_code::solution!(12);

use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u64> {
    let (shapes, queries) = parse_input(input);
    let mut count = 0;

    // Pre-calculate all unique variations for each shape.
    let shape_variations: Vec<Vec<Shape>> =
        shapes.iter().map(|s| s.generate_variations()).collect();

    for (_region_idx, (width, height, required_counts)) in queries.iter().enumerate() {
        // Optimization: Quick area check
        let mut total_area = 0;
        let mut items_to_place = Vec::new();

        for (shape_id, &qty) in required_counts.iter().enumerate() {
            if qty > 0 {
                total_area += shapes[shape_id].area * qty;
                for _ in 0..qty {
                    items_to_place.push(shape_id);
                }
            }
        }

        if total_area > width * height {
            continue;
        }

        // Sort items to place by area (descending) to fail fast.
        // Tie-break with ID to keep identical items adjacent.
        items_to_place.sort_by(|a, b| {
            let area_cmp = shapes[*b].area.cmp(&shapes[*a].area);
            if area_cmp == std::cmp::Ordering::Equal {
                a.cmp(b)
            } else {
                area_cmp
            }
        });

        let mut grid = vec![0u64; *height]; // Using u64 for rows, max width is effectively 64
        if *width > 64 {
            panic!("Grid width > 64 not supported by this optimization");
        }

        if solve_recursive(
            &mut grid,
            *width,
            *height,
            &items_to_place,
            &shape_variations,
            0,
            0,
        ) {
            count += 1;
        }
    }

    Some(count)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Shape {
    points: Vec<(usize, usize)>, // (row, col)
    masks: Vec<u64>,             // Bitmask for each row of the shape
    height: usize,
    width: usize,
    area: usize,
}

impl Shape {
    fn new(lines: &[&str]) -> Self {
        let height = lines.len();
        let width = lines[0].len();
        let mut points = Vec::new();
        let mut area = 0;

        for (r, line) in lines.iter().enumerate() {
            for (c, ch) in line.chars().enumerate() {
                if ch == '#' {
                    points.push((r, c));
                    area += 1;
                }
            }
        }

        let mut masks = vec![0u64; height];
        for &(r, c) in &points {
            masks[r] |= 1 << c;
        }

        Self {
            points,
            masks,
            height,
            width,
            area,
        }
    }

    fn generate_variations(&self) -> Vec<Shape> {
        let mut variations = HashSet::new();
        let mut current = self.clone();

        // 4 rotations
        for _ in 0..4 {
            variations.insert(current.normalize());
            variations.insert(current.flip().normalize());
            current = current.rotate();
        }

        variations.into_iter().collect()
    }

    fn rotate(&self) -> Shape {
        // Rotate 90 degrees clockwise
        let new_height = self.width;
        let new_width = self.height;
        let mut new_points = Vec::new();

        for (r, c) in &self.points {
            new_points.push((*c, self.height - 1 - *r));
        }

        // Rebuild masks
        let mut new_masks = vec![0u64; new_height];
        for &(r, c) in &new_points {
            new_masks[r] |= 1 << c;
        }

        Shape {
            points: new_points,
            masks: new_masks,
            height: new_height,
            width: new_width,
            area: self.area,
        }
    }

    fn flip(&self) -> Shape {
        // Flip horizontally
        let mut new_points = Vec::new();
        for (r, c) in &self.points {
            new_points.push((*r, self.width - 1 - *c));
        }

        let mut new_masks = vec![0u64; self.height];
        for &(r, c) in &new_points {
            new_masks[r] |= 1 << c;
        }

        Shape {
            points: new_points,
            masks: new_masks,
            height: self.height,
            width: self.width,
            area: self.area,
        }
    }

    fn normalize(&self) -> Shape {
        // Shift to top-left (0,0) and sort points
        if self.points.is_empty() {
            return self.clone();
        }

        let min_r = self.points.iter().map(|p| p.0).min().unwrap();
        let min_c = self.points.iter().map(|p| p.1).min().unwrap();

        let mut new_points: Vec<(usize, usize)> = self
            .points
            .iter()
            .map(|(r, c)| (r - min_r, c - min_c))
            .collect();

        new_points.sort();

        let max_r = new_points.iter().map(|p| p.0).max().unwrap();
        let max_c = new_points.iter().map(|p| p.1).max().unwrap();
        let height = max_r + 1;
        let width = max_c + 1;

        let mut new_masks = vec![0u64; height];
        for &(r, c) in &new_points {
            new_masks[r] |= 1 << c;
        }

        Shape {
            points: new_points,
            masks: new_masks,
            height,
            width,
            area: self.area,
        }
    }
}

// Returns true if all items can be placed
fn solve_recursive(
    grid: &mut [u64],
    width: usize,
    height: usize,
    remaining_items: &[usize],
    variations: &[Vec<Shape>],
    start_r: usize,
    start_c: usize,
) -> bool {
    if remaining_items.is_empty() {
        return true;
    }

    let shape_id = remaining_items[0];
    let next_remaining = &remaining_items[1..];

    let available_variations = &variations[shape_id];

    // Symmetry breaking preparation
    let next_is_same = next_remaining
        .first()
        .map_or(false, |&next_id| next_id == shape_id);

    // Iteration logic to support start_r, start_c
    for r in start_r..height {
        // For the first row, start at start_c. For others, start at 0.
        let c_start = if r == start_r { start_c } else { 0 };

        for c in c_start..width {
            for shape in available_variations {
                if r + shape.height > height || c + shape.width > width {
                    continue;
                }

                if can_place(grid, shape, r, c) {
                    place(grid, shape, r, c);

                    let (next_r, next_c) = if next_is_same { (r, c) } else { (0, 0) };

                    if solve_recursive(
                        grid,
                        width,
                        height,
                        next_remaining,
                        variations,
                        next_r,
                        next_c,
                    ) {
                        return true;
                    }
                    remove(grid, shape, r, c);
                }
            }
        }
    }

    false
}

fn can_place(grid: &[u64], shape: &Shape, r: usize, c: usize) -> bool {
    for (dr, &mask) in shape.masks.iter().enumerate() {
        if (grid[r + dr] & (mask << c)) != 0 {
            return false;
        }
    }
    true
}

fn place(grid: &mut [u64], shape: &Shape, r: usize, c: usize) {
    for (dr, &mask) in shape.masks.iter().enumerate() {
        grid[r + dr] |= mask << c;
    }
}

fn remove(grid: &mut [u64], shape: &Shape, r: usize, c: usize) {
    for (dr, &mask) in shape.masks.iter().enumerate() {
        grid[r + dr] &= !(mask << c);
    }
}

fn parse_input(input: &str) -> (Vec<Shape>, Vec<(usize, usize, Vec<usize>)>) {
    let mut shapes = Vec::new();
    let mut queries = Vec::new();

    for block in input.split("\n\n") {
        let block = block.trim();
        if block.is_empty() {
            continue;
        }

        if let Some(first_line) = block.lines().next() {
            if first_line.contains(':')
                && first_line.chars().nth(0).unwrap().is_digit(10)
                && !first_line.contains('x')
            {
                // Shape
                let id_str = first_line.strip_suffix(":").unwrap();
                let _id: usize = id_str.parse().unwrap();

                let grid_lines: Vec<&str> = block.lines().skip(1).collect();
                let shape = Shape::new(&grid_lines);
                if shapes.len() <= _id {
                    shapes.resize(_id + 1, shape.clone());
                }
                shapes[_id] = shape;
            } else {
                // Regions
                for line in block.lines() {
                    if let Some((dims, counts_str)) = line.split_once(": ") {
                        let (w_str, h_str) = dims.split_once('x').unwrap();
                        let w: usize = w_str.parse().unwrap();
                        let h: usize = h_str.parse().unwrap();

                        let counts: Vec<usize> = counts_str
                            .split_whitespace()
                            .map(|s| s.parse().unwrap())
                            .collect();

                        queries.push((w, h, counts));
                    }
                }
            }
        }
    }

    (shapes, queries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
