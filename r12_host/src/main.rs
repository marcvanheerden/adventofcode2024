use std::collections::HashSet;

#[derive(Debug)]
struct Map {
    locs: Vec<u8>,
    width: usize,
    height: usize,
}

#[derive(Debug, PartialEq)]
enum Side {
    Left,
    Right,
    Top,
    Bottom,
}

impl Map {
    fn new(input: &str) -> Self {
        let width = input.find('\n').unwrap();
        let height = input.len() / (width + 1);
        let locs = input
            .chars()
            .filter(|&c| c != '\n')
            .map(|c| c as u8)
            .collect();

        Self {
            locs,
            width,
            height,
        }
    }

    fn get(&self, row: usize, col: usize) -> Option<u8> {
        if (row < self.height) && (col < self.width) {
            return Some(self.locs[row * self.width + col]);
        }

        None
    }

    fn surrounds(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut nbors = Vec::with_capacity(4);

        if row > 0 {
            nbors.push((row - 1, col));
        }

        if row < (self.height - 1) {
            nbors.push((row + 1, col));
        }

        if col > 0 {
            nbors.push((row, col - 1));
        }

        if col < (self.width - 1) {
            nbors.push((row, col + 1));
        }

        nbors
    }

    fn surround_vals(&self, row: usize, col: usize) -> Vec<u8> {
        let mut nbors = Vec::with_capacity(4);

        if row > 0 {
            nbors.push(self.locs[(row - 1) * self.width + col]);
        }

        if row < (self.height - 1) {
            nbors.push(self.locs[(row + 1) * self.width + col]);
        }

        if col > 0 {
            nbors.push(self.locs[row * self.width + col - 1]);
        }

        if col < (self.width - 1) {
            nbors.push(self.locs[row * self.width + col + 1]);
        }

        nbors
    }

    fn perimeter(&self, row: usize, col: usize) -> usize {
        if let Some(val) = self.get(row, col) {
            let mut fences = self
                .surround_vals(row, col)
                .into_iter()
                .filter(|&s| s != val)
                .count();

            if row == 0 {
                fences += 1;
            }
            if row == (self.height - 1) {
                fences += 1;
            }
            if col == 0 {
                fences += 1;
            }
            if col == (self.width - 1) {
                fences += 1;
            }

            return fences;
        }

        0
    }

    fn perimeter_sides(&self, row: usize, col: usize) -> Vec<Side> {
        if let Some(val) = self.get(row, col) {
            let mut sides = Vec::new();

            if row == 0 || self.get(row - 1, col).unwrap() != val {
                sides.push(Side::Top);
            }

            if row == (self.height - 1) || self.get(row + 1, col).unwrap() != val {
                sides.push(Side::Bottom);
            }
            if col == 0 || self.get(row, col - 1).unwrap() != val {
                sides.push(Side::Left);
            }
            if col == (self.width - 1) || self.get(row, col + 1).unwrap() != val {
                sides.push(Side::Right);
            }

            return sides;
        }

        Vec::new()
    }

    fn sides(&self, row: usize, col: usize) -> usize {
        let mut point_sides = self.perimeter_sides(row, col);
        let plant = self.get(row, col).unwrap();

        // if it is a fence
        // but not if there is a piece directly above, of the same plant, with the same edge

        if point_sides.contains(&Side::Left) {
            if let Some(above_plant) = self.get(row.wrapping_sub(1), col) {
                if plant == above_plant && self.perimeter_sides(row - 1, col).contains(&Side::Left)
                {
                    point_sides.retain(|s| *s != Side::Left);
                }
            }
        }

        if point_sides.contains(&Side::Right) {
            if let Some(above_plant) = self.get(row.wrapping_sub(1), col) {
                if plant == above_plant && self.perimeter_sides(row - 1, col).contains(&Side::Right)
                {
                    point_sides.retain(|s| *s != Side::Right);
                }
            }
        }

        if point_sides.contains(&Side::Top) {
            if let Some(left_plant) = self.get(row, col.wrapping_sub(1)) {
                if plant == left_plant && self.perimeter_sides(row, col - 1).contains(&Side::Top) {
                    point_sides.retain(|s| *s != Side::Top);
                }
            }
        }

        if point_sides.contains(&Side::Bottom) {
            if let Some(left_plant) = self.get(row, col.wrapping_sub(1)) {
                if plant == left_plant && self.perimeter_sides(row, col - 1).contains(&Side::Bottom)
                {
                    point_sides.retain(|s| *s != Side::Bottom);
                }
            }
        }

        point_sides.len()
    }

    fn flood_fill(&self) -> Vec<HashSet<(usize, usize)>> {
        let mut gardens = Vec::new();
        let mut current = HashSet::new();

        while gardens
            .iter()
            .map(|s: &HashSet<(usize, usize)>| s.len())
            .sum::<usize>()
            < self.width * self.height
        {
            // seed a value to a new garden if empty
            if current.is_empty() {
                let all_locs: Vec<_> = gardens.iter().flatten().cloned().collect();
                'outer: for row in 0..self.height {
                    for col in 0..self.width {
                        if !all_locs.contains(&(row, col)) {
                            current.insert((row, col));
                            break 'outer;
                        }
                    }
                }
            }

            let current_copy = current.clone();
            for (row, col) in current_copy.iter() {
                let plant = self.get(*row, *col).unwrap();
                let nbors = self.surrounds(*row, *col);
                let same_plant = nbors
                    .into_iter()
                    .filter(|(r, c)| self.get(*r, *c).unwrap() == plant);
                current.extend(same_plant);
            }
            if current_copy.len() == current.len() {
                gardens.push(current);
                current = HashSet::new();
            }
        }

        gardens
    }

    fn solution(&self) -> (usize, usize) {
        let gardens = self.flood_fill();

        let part1 = gardens
            .iter()
            .map(|s| s.len() * s.iter().map(|(r, c)| self.perimeter(*r, *c)).sum::<usize>())
            .sum();

        let part2 = gardens
            .into_iter()
            .map(|s| s.len() * s.iter().map(|(r, c)| self.sides(*r, *c)).sum::<usize>())
            .sum();

        (part1, part2)
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let map = Map::new(input);

    dbg!(map.solution());
}
