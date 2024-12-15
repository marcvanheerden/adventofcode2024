use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Dir {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Copy, Clone)]
enum Obstacle {
    LeftBox,
    RightBox,
    Wall,
}

#[derive(Debug)]
struct Map2 {
    obstacles: HashMap<(usize, usize), Obstacle>,
    bot: (usize, usize),
    instructions: Vec<Dir>,
}

impl Map2 {
    fn new(input: &str) -> Self {
        let (map_str, instr_str) = input.split_once("\n\n").unwrap();

        let mut obstacles = HashMap::new();
        let mut bot = (0, 0);
        for (row, line) in map_str.lines().enumerate() {
            for (col, chr) in line.chars().enumerate() {
                match chr {
                    '@' => {
                        bot = (row, col * 2);
                    }
                    '#' => {
                        obstacles.insert((row, col * 2), Obstacle::Wall);
                        obstacles.insert((row, col * 2 + 1), Obstacle::Wall);
                    }
                    'O' => {
                        obstacles.insert((row, col * 2), Obstacle::LeftBox);
                        obstacles.insert((row, col * 2 + 1), Obstacle::RightBox);
                    }
                    _ => {}
                }
            }
        }

        let instructions = instr_str
            .chars()
            .filter_map(|chr| match chr {
                '^' => Some(Dir::North),
                '>' => Some(Dir::East),
                'v' => Some(Dir::South),
                '<' => Some(Dir::West),
                _ => None,
            })
            .rev()
            .collect();

        Self {
            obstacles,
            bot,
            instructions,
        }
    }

    fn next_pos(pos: (usize, usize), dir: Dir) -> (usize, usize) {
        match dir {
            Dir::North => (pos.0 - 1, pos.1),
            Dir::East => (pos.0, pos.1 + 1),
            Dir::South => (pos.0 + 1, pos.1),
            Dir::West => (pos.0, pos.1 - 1),
        }
    }

    fn can_clear_space(&mut self, pos: (usize, usize), dir: Dir, internal: bool) -> bool {
        let space_occupied = self.obstacles.get(&pos);

        match space_occupied {
            Some(Obstacle::Wall) => false,
            Some(Obstacle::LeftBox) => {
                let next_pos = Self::next_pos(pos, dir);
                let mut outcome = self.can_clear_space(next_pos, dir, false);

                if !internal && [Dir::North, Dir::South].contains(&dir) {
                    let other_half = (pos.0, pos.1 + 1);
                    outcome = outcome && self.can_clear_space(other_half, dir, true);
                }

                outcome
            }
            Some(Obstacle::RightBox) => {
                let next_pos = Self::next_pos(pos, dir);
                let mut outcome = self.can_clear_space(next_pos, dir, false);

                if !internal && [Dir::North, Dir::South].contains(&dir) {
                    let other_half = (pos.0, pos.1 - 1);
                    outcome = outcome && self.can_clear_space(other_half, dir, true);
                }

                outcome
            }
            None => true,
        }
    }

    fn do_clear_space(&mut self, pos: (usize, usize), dir: Dir, internal: bool) {
        let space_occupied = self.obstacles.get(&pos);

        match space_occupied {
            Some(Obstacle::Wall) => unreachable!(),
            Some(Obstacle::LeftBox) => {
                let next_pos = Self::next_pos(pos, dir);
                let other_half = (pos.0, pos.1 + 1);
                self.do_clear_space(next_pos, dir, false);
                if let Some(val) = self.obstacles.remove(&pos) {
                    self.obstacles.insert(next_pos, val);
                }
                if !internal && [Dir::North, Dir::South].contains(&dir) {
                    self.do_clear_space(other_half, dir, true);
                }
            }
            Some(Obstacle::RightBox) => {
                let next_pos = Self::next_pos(pos, dir);
                let other_half = (pos.0, pos.1 - 1);
                self.do_clear_space(next_pos, dir, false);
                if let Some(val) = self.obstacles.remove(&pos) {
                    self.obstacles.insert(next_pos, val);
                }
                if !internal && [Dir::North, Dir::South].contains(&dir) {
                    self.do_clear_space(other_half, dir, true);
                }
            }
            None => {}
        }
    }

    fn step(&mut self) -> bool {
        if let Some(dir) = self.instructions.pop() {
            let next_pos = Self::next_pos(self.bot, dir);
            if self.can_clear_space(next_pos, dir, false) {
                self.do_clear_space(next_pos, dir, false);
                self.bot = next_pos;
            }
            true
        } else {
            false
        }
    }

    fn score(&self) -> usize {
        self.obstacles
            .iter()
            .filter_map(|(pos, movable)| match *movable {
                Obstacle::LeftBox => Some(pos.0 * 100 + pos.1),
                _ => None,
            })
            .sum::<usize>()
    }

    fn display(&self, width: usize, height: usize) {
        println!();
        println!();
        println!();
        println!();
        for y in 0..height {
            println!();
            for x in 0..width {
                if self.bot == (y, x) {
                    print!("@");
                    continue;
                }

                match self.obstacles.get(&(y, x)) {
                    Some(Obstacle::LeftBox) => print!("["),
                    Some(Obstacle::RightBox) => print!("]"),
                    Some(Obstacle::Wall) => print!("#"),
                    None => print!(" "),
                }
            }
        }
    }
}

#[derive(Debug)]
struct Map {
    obstacles: HashMap<(usize, usize), bool>,
    bot: (usize, usize),
    instructions: Vec<Dir>,
}

impl Map {
    fn new(input: &str) -> Self {
        let (map_str, instr_str) = input.split_once("\n\n").unwrap();

        let mut obstacles = HashMap::new();
        let mut bot = (0, 0);
        for (row, line) in map_str.lines().enumerate() {
            for (col, chr) in line.chars().enumerate() {
                match chr {
                    '@' => {
                        bot = (row, col);
                    }
                    '#' => {
                        obstacles.insert((row, col), false);
                    }
                    'O' => {
                        obstacles.insert((row, col), true);
                    }
                    _ => {}
                }
            }
        }

        let instructions = instr_str
            .chars()
            .filter_map(|chr| match chr {
                '^' => Some(Dir::North),
                '>' => Some(Dir::East),
                'v' => Some(Dir::South),
                '<' => Some(Dir::West),
                _ => None,
            })
            .rev()
            .collect();

        Self {
            obstacles,
            bot,
            instructions,
        }
    }

    fn next_pos(pos: (usize, usize), dir: Dir) -> (usize, usize) {
        match dir {
            Dir::North => (pos.0 - 1, pos.1),
            Dir::East => (pos.0, pos.1 + 1),
            Dir::South => (pos.0 + 1, pos.1),
            Dir::West => (pos.0, pos.1 - 1),
        }
    }

    fn clear_space(&mut self, pos: (usize, usize), dir: Dir) -> bool {
        let space_occupied = self.obstacles.get(&pos);
        if space_occupied.is_none() {
            return true;
        }

        let potentially_movable = *space_occupied.unwrap();

        if potentially_movable {
            let next_pos = Self::next_pos(pos, dir);
            let space_is_clear = self.clear_space(next_pos, dir);

            if space_is_clear {
                if let Some(val) = self.obstacles.remove(&pos) {
                    self.obstacles.insert(next_pos, val);
                }
                return true;
            }
        }
        false
    }

    fn step(&mut self) -> bool {
        if let Some(dir) = self.instructions.pop() {
            let next_pos = Self::next_pos(self.bot, dir);
            if self.clear_space(next_pos, dir) {
                self.bot = next_pos;
            }
            true
        } else {
            false
        }
    }

    fn score(&self) -> usize {
        self.obstacles
            .iter()
            .filter_map(|(pos, movable)| {
                if *movable {
                    Some(pos.0 * 100 + pos.1)
                } else {
                    None
                }
            })
            .sum::<usize>()
    }

    fn display(&self, width: usize, height: usize) {
        println!();
        println!();
        println!();
        println!();
        for y in 0..height {
            println!();
            for x in 0..width {
                if let Some(val) = self.obstacles.get(&(y, x)) {
                    if *val {
                        print!("O");
                    } else {
                        print!("#");
                    }
                } else if self.bot == (y, x) {
                    print!("@");
                } else {
                    print!(" ");
                }
            }
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut map = Map::new(input);
    map.display(10, 10);

    loop {
        if !map.step() {
            dbg!(map.score());
            break;
        }
        map.display(10, 9);
    }

    let mut map = Map2::new(input);
    map.display(14, 8);

    loop {
        map.display(14, 8);
        if !map.step() {
            dbg!(map.score());
            map.display(14, 8);
            break;
        }
    }
}
