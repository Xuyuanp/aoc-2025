advent_of_code::solution!(9);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile(u64, u64);

impl TryFrom<&str> for Tile {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split(',');
        let x = parts
            .next()
            .ok_or("Missing x coordinate")?
            .parse::<u64>()
            .map_err(|_| "Invalid x coordinate")?;
        let y = parts
            .next()
            .ok_or("Missing y coordinate")?
            .parse::<u64>()
            .map_err(|_| "Invalid y coordinate")?;
        Ok(Self::new(x, y))
    }
}

impl Tile {
    pub fn new(x: u64, y: u64) -> Self {
        Self(x, y)
    }

    pub fn x(&self) -> u64 {
        self.0
    }

    pub fn y(&self) -> u64 {
        self.1
    }

    pub fn area(&self, other: &Self) -> u64 {
        let width = self.0.abs_diff(other.0) + 1;
        let height = self.1.abs_diff(other.1) + 1;
        width * height
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let tiles: Vec<Tile> = input
        .lines()
        .filter_map(|line| Tile::try_from(line).ok())
        .collect();

    tiles
        .iter()
        .enumerate()
        .flat_map(|(i, t1)| tiles.iter().skip(i + 1).map(move |t2| t1.area(t2)))
        .max()
}

pub fn part_two(input: &str) -> Option<u64> {
    let vertices: Vec<Tile> = input
        .lines()
        .filter_map(|line| Tile::try_from(line).ok())
        .collect();

    // The vertices form a polygon.
    // We want the largest rectangle with opposite corners in `vertices`
    // such that the rectangle is fully contained in the polygon.
    //
    // A rectangle R (defined by v1, v2) is inside the polygon P if:
    // 1. The other two corners (v1.x, v2.y) and (v2.x, v1.y) are inside P (or on boundary).
    // 2. The center of R is inside P (to ensure we are not "bridging" across a gap).
    // 3. No edge of P intersects the interior of R?
    //    Actually, for a rectilinear polygon, checking that all 4 corners are in P + center is in P
    //    might not be enough if the polygon has a "hole" or "indentation" that the rectangle covers.
    //
    //    Correct condition for R \subseteq P:
    //    - All 4 corners in P.
    //    - Center in P.
    //    - No vertex of P is strictly inside R.
    //    Since P is simple and rectilinear, if no vertex is inside R and the boundary/center checks pass,
    //    then R cannot "cross" a boundary without containing a vertex (or having a corner outside).

    let n = vertices.len();
    let mut max_area = 0;

    for i in 0..n {
        for j in i + 1..n {
            let v1 = vertices[i];
            let v2 = vertices[j];

            // If x or y coordinates match, it's a line, not a rectangle (area 0 or 1 width? problem says "area").
            // Area = width * height. If x1==x2, width=1?
            // "area of 24 between 2,5 and 9,7" -> (9-2+1)*(7-5+1) = 8 * 3 = 24.
            // So width/height is abs_diff + 1. Even a line has area > 0?
            // "thin rectangle with an area of only 6 between 7,3 and 2,3" -> (7-2+1)*(3-3+1) = 6 * 1.
            // So yes, even collinear points form a valid rectangle (segment).
            // But problem says "rectangle that uses red tiles for two of its opposite corners".
            // A segment is a degenerate rectangle.

            let current_area = v1.area(&v2);
            if current_area <= max_area {
                continue;
            }

            // Check if valid
            if is_valid_rectangle(&v1, &v2, &vertices) {
                max_area = current_area;
            }
        }
    }

    Some(max_area)
}

fn is_valid_rectangle(v1: &Tile, v2: &Tile, polygon: &[Tile]) -> bool {
    // Form the 4 corners
    let min_x = v1.x().min(v2.x());
    let max_x = v1.x().max(v2.x());
    let min_y = v1.y().min(v2.y());
    let max_y = v1.y().max(v2.y());

    // Corner 1: (min_x, min_y)
    // Corner 2: (max_x, min_y)
    // Corner 3: (max_x, max_y)
    // Corner 4: (min_x, max_y)

    // Check 1: All 4 corners must be inside or on boundary.
    // v1 and v2 are vertices, so they are on boundary.
    // We must check the other two corners explicitly.
    // Actually, checking all 4 is safer and doesn't hurt.
    let c1 = Tile::new(min_x, min_y);
    let c2 = Tile::new(max_x, min_y);
    let c3 = Tile::new(max_x, max_y);
    let c4 = Tile::new(min_x, max_y);

    if !is_point_in_polygon(&c1, polygon) {
        return false;
    }
    if !is_point_in_polygon(&c2, polygon) {
        return false;
    }
    if !is_point_in_polygon(&c3, polygon) {
        return false;
    }
    if !is_point_in_polygon(&c4, polygon) {
        return false;
    }

    // Check 2: No polygon vertex is strictly inside the rectangle.
    // Strictly inside: min_x < vx < max_x AND min_y < vy < max_y
    for v in polygon {
        if v.x() > min_x && v.x() < max_x && v.y() > min_y && v.y() < max_y {
            return false;
        }
    }

    // Check 3: No polygon edge intersects the interior of the rectangle.
    // This catches invalid regions ("holes" or "cuts") that pass completely through
    // the rectangle without leaving a vertex inside (or if vertices are on boundary).
    if edges_intersect_rect(min_x, max_x, min_y, max_y, polygon) {
        return false;
    }

    // Check 4: Center is inside polygon.
    // To avoid floating point, we can pick a point (min_x + 0.5, min_y + 0.5).
    // Or just check if the "average" integer coordinate is inside, but "between" tiles is safer.
    // The grid is discrete checks, "inside this loop of red and green tiles".
    // "all of the tiles inside this loop ... are also green".
    // So we just need to check if ANY tile inside the rectangle is effectively "part of the interior".
    //
    // If the rectangle has width 1 or height 1, it has no "interior" tiles (only boundary).
    // In that case, since corners are in P and no vertex is strictly in R (vacuously true),
    // we just need to make sure the edge itself is in P.
    // Since it's a rectilinear polygon, if endpoints are in P and no vertex is in betweeen,
    // the segment is in P ONLY IF logic holds.
    // Actually, for "thin" rectangle (segment), we need to check if the mid-point of the segment is in P.

    let mid_x = (min_x + max_x) as f64 / 2.0;
    let mid_y = (min_y + max_y) as f64 / 2.0;

    // Use ray casting for center
    is_point_in_polygon_f64(mid_x, mid_y, polygon)
}

fn is_point_in_polygon(pt: &Tile, polygon: &[Tile]) -> bool {
    // Standard ray casting algo for integer points
    // Ray to the right: (pt.x, pt.y) -> (infinity, pt.y)
    // Count intersections with polygon edges.
    // Edges are (p[i], p[i+1])

    let mut inside = false;
    let n = polygon.len();
    for i in 0..n {
        let p1 = &polygon[i];
        let p2 = &polygon[(i + 1) % n];

        // Check if point is ON the edge (boundary is inclusion)
        if on_segment(pt, p1, p2) {
            return true;
        }

        // Ray casting
        // Check if edge intersects the ray y = pt.y, x >= pt.x
        // Edge must span the y-coordinate of pt
        // Use > for one and <= for other to handle vertices cleanly (simulating epsilon shift)
        if (p1.y() > pt.y()) != (p2.y() > pt.y()) {
            // intersection x
            // x = x1 + (y - y1) * (x2 - x1) / (y2 - y1)
            // We need intersection_x >= pt.x
            // careful with unsigned subtraction, cast to i64 or f64

            let p1x = p1.x() as f64;
            let p1y = p1.y() as f64;
            let p2x = p2.x() as f64;
            let p2y = p2.y() as f64;
            let pty = pt.y() as f64;

            let intersect_x = p1x + (pty - p1y) * (p2x - p1x) / (p2y - p1y);
            if intersect_x >= pt.x() as f64 {
                inside = !inside;
            }
        }
    }
    inside
}

fn is_point_in_polygon_f64(x: f64, y: f64, polygon: &[Tile]) -> bool {
    let mut inside = false;
    let n = polygon.len();
    for i in 0..n {
        let p1 = &polygon[i];
        let p2 = &polygon[(i + 1) % n];

        // Ray casting logic
        if (p1.y() as f64 > y) != (p2.y() as f64 > y) {
            let p1x = p1.x() as f64;
            let p1y = p1.y() as f64;
            let p2x = p2.x() as f64;
            let p2y = p2.y() as f64;

            let intersect_x = p1x + (y - p1y) * (p2x - p1x) / (p2y - p1y);
            if intersect_x > x {
                inside = !inside;
            }
        }
    }
    inside
}

fn on_segment(p: &Tile, a: &Tile, b: &Tile) -> bool {
    // Check if p is on segment a-b
    // Since edges are rectilinear, simpler check?
    // The general case:
    // Cross product approach or min/max check if rectilinear
    // Collinear check: (b.x - a.x) * (p.y - a.y) == (b.y - a.y) * (p.x - a.x)
    // And between check

    let crossp = (b.x() as i128 - a.x() as i128) * (p.y() as i128 - a.y() as i128)
        - (b.y() as i128 - a.y() as i128) * (p.x() as i128 - a.x() as i128);
    if crossp != 0 {
        return false;
    }

    let min_x = a.x().min(b.x());
    let max_x = a.x().max(b.x());
    let min_y = a.y().min(b.y());
    let max_y = a.y().max(b.y());

    p.x() >= min_x && p.x() <= max_x && p.y() >= min_y && p.y() <= max_y
}

fn edges_intersect_rect(min_x: u64, max_x: u64, min_y: u64, max_y: u64, polygon: &[Tile]) -> bool {
    let n = polygon.len();
    for i in 0..n {
        let p1 = &polygon[i];
        let p2 = &polygon[(i + 1) % n];

        // Check intersection of segment p1-p2 with the rectangle INTERIOR.
        // Segment is vertical or horizontal.

        if p1.x() == p2.x() {
            // Vertical edge at x = p1.x()
            let x = p1.x();
            // strictly between min_x and max_x
            if x > min_x && x < max_x {
                // Check y overlap
                let seg_min_y = p1.y().min(p2.y());
                let seg_max_y = p1.y().max(p2.y());

                // Does (seg_min, seg_max) overlap (min_y, max_y)?
                // Interval overlap: max(A.start, B.start) < min(A.end, B.end)
                // We want strict interior overlap, so strictly inside y range?
                // If the edge just looks into the box, it "intersects interior".
                let overlap_start = seg_min_y.max(min_y);
                let overlap_end = seg_max_y.min(max_y);

                if overlap_start < overlap_end {
                    return true;
                }
            }
        } else {
            // Horizontal edge at y = p1.y()
            let y = p1.y();
            // strictly between min_y and max_y
            if y > min_y && y < max_y {
                // Check x overlap
                let seg_min_x = p1.x().min(p2.x());
                let seg_max_x = p1.x().max(p2.x());

                let overlap_start = seg_min_x.max(min_x);
                let overlap_end = seg_max_x.min(max_x);

                if overlap_start < overlap_end {
                    return true;
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area() {
        let t1 = Tile::new(2, 5);
        let t2 = Tile::new(9, 7);
        assert_eq!(t1.area(&t2), 24);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
