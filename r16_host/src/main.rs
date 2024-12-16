use std::collections::{HashMap, HashSet, VecDeque};

const TURN_COST: u32 = 1000;
const MOVE_COST: u32 = 1;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn opposite(&self) -> Self {
        match self {
            Dir::North => Dir::South,
            Dir::East => Dir::West,
            Dir::South => Dir::North,
            Dir::West => Dir::East,
        }
    }
}

#[derive(Debug, Clone)]
struct Reindeer {
    acc_cost: u32,
    pos: (u8, u8),
    dir: Dir,
}

#[derive(Debug)]
struct Maze {
    walls: HashSet<(u8, u8)>,
    end: (u8, u8),
}

impl Maze {
    fn next_moves(&self, rd: &Reindeer) -> Vec<(u8, u8, Dir)> {
        let moves = [
            (rd.pos.0 - 1, rd.pos.1, Dir::North),
            (rd.pos.0, rd.pos.1 + 1, Dir::East),
            (rd.pos.0 + 1, rd.pos.1, Dir::South),
            (rd.pos.0, rd.pos.1 - 1, Dir::West),
        ];

        moves
            .into_iter()
            .filter(|(x, y, d)| !self.walls.contains(&(*x, *y)) && rd.dir.opposite() != *d)
            .collect()
    }
}

fn start(input: &str) -> (Maze, Reindeer) {
    let mut walls = HashSet::new();
    let mut end = (0, 0);
    let mut pos = (0, 0);
    let dir = Dir::East;
    let acc_cost = 0;

    for (row, line) in input.lines().enumerate() {
        for (col, chr) in line.chars().enumerate() {
            match chr {
                '#' => {
                    walls.insert((row as u8, col as u8));
                }
                'S' => {
                    pos = (row as u8, col as u8);
                }
                'E' => {
                    end = (row as u8, col as u8);
                }
                _ => {}
            }
        }
    }

    let maze = Maze { walls, end };

    let reindeer = Reindeer { pos, dir, acc_cost };

    (maze, reindeer)
}

fn cheapest_path(maze: &Maze, reindeer: &Reindeer) -> u32 {
    let mut tasks = VecDeque::new();
    tasks.push_front(reindeer.clone());

    let mut visited = HashMap::new();

    let mut best_score_so_far = u32::MAX;

    while !tasks.is_empty() {
        for _ in 0..tasks.len() {
            let rd = tasks.pop_front().unwrap();
            if rd.pos == maze.end {
                if rd.acc_cost < best_score_so_far {
                    best_score_so_far = rd.acc_cost;
                }
                continue;
            }

            if rd.acc_cost >= best_score_so_far {
                continue;
            }

            if let Some(score) = visited.get(&(rd.pos.0, rd.pos.1, rd.dir.clone())) {
                if *score < rd.acc_cost {
                    continue;
                }
            }
            visited
                .entry((rd.pos.0, rd.pos.1, rd.dir.clone()))
                .and_modify(|score| *score = rd.acc_cost)
                .or_insert(rd.acc_cost);

            for (x, y, dir) in maze.next_moves(&rd) {
                let mut acc_cost = rd.acc_cost + MOVE_COST;

                if dir != rd.dir {
                    acc_cost += TURN_COST;
                }

                tasks.push_back(Reindeer {
                    pos: (x, y),
                    dir,
                    acc_cost,
                })
            }
        }
    }

    best_score_so_far
}

fn cheap_path_tiles(maze: &Maze, reindeer: &Reindeer, cheapest: u32) -> usize {
    let mut tasks = VecDeque::new();
    tasks.push_front((reindeer.clone(), vec![reindeer.pos]));

    let mut visited = HashMap::new();
    let mut seats = HashSet::new();

    while !tasks.is_empty() {
        for _ in 0..tasks.len() {
            let (rd, hist) = tasks.pop_front().unwrap();
            if rd.acc_cost > cheapest {
                continue;
            }

            if rd.pos == maze.end {
                seats.extend(hist.clone());
            }

            if let Some(score) = visited.get(&(rd.pos.0, rd.pos.1, rd.dir.clone())) {
                if *score < rd.acc_cost {
                    continue;
                }
            }
            visited
                .entry((rd.pos.0, rd.pos.1, rd.dir.clone()))
                .and_modify(|score| *score = rd.acc_cost)
                .or_insert(rd.acc_cost);

            for (x, y, dir) in maze.next_moves(&rd) {
                let mut acc_cost = rd.acc_cost + MOVE_COST;

                if dir != rd.dir {
                    acc_cost += TURN_COST;
                }
                let mut new_hist = hist.clone();
                new_hist.push((x, y));
                tasks.push_back((
                    Reindeer {
                        pos: (x, y),
                        dir,
                        acc_cost,
                    },
                    new_hist,
                ))
            }
        }
    }

    seats.len()
}

fn main() {
    let input = include_str!("../input.txt");
    let (maze, reindeer) = start(input);
    let cheapest = cheapest_path(&maze, &reindeer);
    dbg!(&cheapest);
    dbg!(cheap_path_tiles(&maze, &reindeer, cheapest));
}
