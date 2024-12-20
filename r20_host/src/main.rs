use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
struct Maze {
    walls: HashSet<(u8, u8)>,
    start: (u8, u8),
    end: (u8, u8),
    bounds: (u8, u8),
}

#[derive(Debug, Clone, PartialEq)]
struct Racer {
    pos: (u8, u8),
    budget: u8,
    cheat_paths: Vec<Vec<(u8, u8)>>,
}

impl Racer {
    fn can_stack(&self, budget: u8) -> bool {
        self.budget == 0
            && !self.cheat_paths.is_empty()
            && self.cheat_paths[0].len() == (budget + 1) as usize
    }
}

const WALL_COST: u8 = 1;

impl Maze {
    fn new(input: &str) -> Self {
        let mut walls = HashSet::new();
        let mut start = (0, 0);
        let mut end = (0, 0);
        let mut max_y = 0;
        let mut max_x = 0;

        for (y, line) in input.lines().enumerate() {
            for (x, chr) in line.chars().enumerate() {
                match chr {
                    '#' => {
                        walls.insert((y as u8, x as u8));
                    }
                    'S' => {
                        start = (y as u8, x as u8);
                    }
                    'E' => {
                        end = (y as u8, x as u8);
                    }
                    _ => {}
                }
                max_x = x;
            }
            max_y = y;
        }

        Maze {
            walls,
            start,
            end,
            bounds: (max_y as u8, max_x as u8),
        }
    }

    fn cost(&self, pos: &(u8, u8)) -> u8 {
        if self.walls.contains(pos) {
            WALL_COST
        } else {
            0
        }
    }

    fn next_pos(&self, racer: &Racer) -> Vec<((u8, u8), u8)> {
        let mut next = Vec::with_capacity(4);

        if racer.pos.0 > 0 {
            let next_candidate = (racer.pos.0 - 1, racer.pos.1);
            let next_cost = self.cost(&next_candidate);
            if racer.budget >= next_cost {
                next.push((next_candidate, next_cost));
            }
        }

        if racer.pos.1 > 0 {
            let next_candidate = (racer.pos.0, racer.pos.1 - 1);
            let next_cost = self.cost(&next_candidate);
            if racer.budget >= next_cost {
                next.push((next_candidate, next_cost));
            }
        }

        if racer.pos.0 < self.bounds.0 {
            let next_candidate = (racer.pos.0 + 1, racer.pos.1);
            let next_cost = self.cost(&next_candidate);
            if racer.budget >= next_cost {
                next.push((next_candidate, next_cost));
            }
        }

        if racer.pos.1 < self.bounds.1 {
            let next_candidate = (racer.pos.0, racer.pos.1 + 1);
            let next_cost = self.cost(&next_candidate);
            if racer.budget >= next_cost {
                next.push((next_candidate, next_cost));
            }
        }

        next
    }

    fn fastest_traversal(&self, budget: u8) -> u32 {
        let mut queue = VecDeque::new();
        queue.push_back(Racer {
            pos: self.start,
            budget,
            cheat_paths: Vec::with_capacity(0),
        });

        // keep a history of cheatless traversals as the cutoff
        // for a traversal to be allowed to continue
        let mut cheatless_history = HashSet::new();

        let mut steps = 0;

        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let racer = queue.pop_front().unwrap();

                if racer.pos == self.end {
                    return steps;
                }

                if cheatless_history.contains(&racer.pos) {
                    continue;
                } else if racer.budget == budget {
                    cheatless_history.insert(racer.pos);
                }

                for (next_pos, next_cost) in self.next_pos(&racer) {
                    queue.push_back(Racer {
                        pos: next_pos,
                        budget: racer.budget - next_cost,
                        cheat_paths: Vec::with_capacity(0),
                    });
                }
            }
            steps += 1;
        }

        steps
    }

    fn cheat_paths(&self, budget: u8, max_steps: u32) -> u32 {
        let mut queue = VecDeque::new();
        queue.push_back(Racer {
            pos: self.start,
            budget,
            cheat_paths: Vec::new(),
        });

        // keep a history of cheatless traversals as the cutoff
        // for a traversal to be allowed to continue
        let mut cheatless_history = HashSet::new();

        let mut steps = 0;
        let mut paths = HashSet::new();

        while !queue.is_empty() && steps <= max_steps {
            dbg!(steps);
            let mut next_queue: Vec<Racer> = Vec::new();
            while !queue.is_empty() {
                let racer = queue.pop_front().unwrap();

                if racer.pos == self.end {
                    paths.extend(racer.cheat_paths);
                    continue;
                }

                let manhattan_dist_from_end =
                    self.end.0.abs_diff(racer.pos.0) + self.end.1.abs_diff(racer.pos.1);
                if manhattan_dist_from_end as u32 > (max_steps - steps) {
                    continue;
                }

                if cheatless_history.contains(&racer.pos) {
                    continue;
                } else if racer.budget == budget {
                    cheatless_history.insert(racer.pos);
                }

                for (next_pos, next_cost) in self.next_pos(&racer) {
                    let mut next_cheat_paths = racer.cheat_paths.clone();

                    if !next_cheat_paths.is_empty()
                        && (next_cheat_paths[0].len() < (budget + 1) as usize)
                        && (next_cost > 0 || !next_cheat_paths[0].is_empty())
                    {
                        next_cheat_paths[0].push(next_pos);
                    } else if next_cost > 0 {
                        next_cheat_paths.push(Vec::with_capacity(2));
                        next_cheat_paths[0].push(next_pos);
                    }

                    let next_racer = Racer {
                        pos: next_pos,
                        budget: racer.budget - next_cost,
                        cheat_paths: next_cheat_paths,
                    };

                    let mut found = false;

                    // stack similar tasks by combining their cheat paths
                    if next_racer.can_stack(budget) {
                        for racer_mut in next_queue.iter_mut() {
                            if racer_mut.pos == next_racer.pos && racer_mut.can_stack(budget) {
                                found = true;
                                for move_racer in next_racer.cheat_paths.iter() {
                                    if !racer_mut.cheat_paths.contains(move_racer) {
                                        racer_mut.cheat_paths.push(move_racer.clone());
                                    }
                                }
                                break;
                            }
                        }
                    }

                    if !found {
                        next_queue.push(next_racer);
                    }
                }
            }
            queue.extend(next_queue);
            steps += 1;
        }

        paths.len() as u32
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let maze = Maze::new(input);
    let fastest_no_cheat = maze.fastest_traversal(0);
    dbg!(fastest_no_cheat);
    dbg!(maze.cheat_paths(1, fastest_no_cheat.saturating_sub(100)));
}
