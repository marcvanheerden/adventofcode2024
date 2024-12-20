use std::collections::{HashMap, HashSet, VecDeque};

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
    cheat_start: (u8, u8),
    cheat_path_keys: HashSet<u32>,
}

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
            1
        } else {
            0
        }
    }

    fn next_pos(&self, racer: &Racer) -> Vec<((u8, u8), u8)> {
        let mut next = Vec::with_capacity(4);

        if racer.pos.0 > 0 {
            let next_candidate = (racer.pos.0 - 1, racer.pos.1);
            let next_cost = self.cost(&next_candidate);
            if next_cost == 0 || racer.budget > next_cost {
                next.push((next_candidate, next_cost));
            }
        }

        if racer.pos.1 > 0 {
            let next_candidate = (racer.pos.0, racer.pos.1 - 1);
            let next_cost = self.cost(&next_candidate);
            if next_cost == 0 || racer.budget > next_cost {
                next.push((next_candidate, next_cost));
            }
        }

        if racer.pos.0 < self.bounds.0 {
            let next_candidate = (racer.pos.0 + 1, racer.pos.1);
            let next_cost = self.cost(&next_candidate);
            if next_cost == 0 || racer.budget > next_cost {
                next.push((next_candidate, next_cost));
            }
        }

        if racer.pos.1 < self.bounds.1 {
            let next_candidate = (racer.pos.0, racer.pos.1 + 1);
            let next_cost = self.cost(&next_candidate);
            if next_cost == 0 || racer.budget > next_cost {
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
            cheat_start: (u8::MAX, u8::MAX),
            cheat_path_keys: HashSet::new(),
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
                        cheat_start: (u8::MAX, u8::MAX),
                        cheat_path_keys: HashSet::new(),
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
            cheat_start: (u8::MAX, u8::MAX),
            cheat_path_keys: HashSet::new(),
        });

        let mut steps = 0;
        let mut paths: HashMap<(u8, u8, u8, u8), u32> = HashMap::new();
        let mut path_count = 0;
        let mut eligible_path_keys = HashSet::new();

        // keep a history of cheatless traversals as the cutoff
        // for a traversal to be allowed to continue
        let mut cheatless_history = HashSet::new();

        while !queue.is_empty() && steps <= max_steps {
            dbg!(steps);
            let mut next_queue: Vec<Racer> = Vec::new();
            while !queue.is_empty() {
                let racer = queue.pop_front().unwrap();

                // terminal condition
                if racer.pos == self.end {
                    eligible_path_keys.extend(racer.cheat_path_keys);
                    continue;
                }

                if cheatless_history.contains(&racer.pos) {
                    continue;
                } else if racer.budget == budget {
                    cheatless_history.insert(racer.pos);
                }

                for (next_pos, next_cost) in self.next_pos(&racer) {
                    // hasn't cheated yet
                    if racer.budget == budget {
                        let next_cheat_start = if next_cost > 0 {
                            racer.pos
                        } else {
                            (u8::MAX, u8::MAX)
                        };

                        let next_racer = Racer {
                            pos: next_pos,
                            budget: racer.budget - next_cost,
                            cheat_start: next_cheat_start,
                            cheat_path_keys: HashSet::new(),
                        };

                        if !next_queue.contains(&next_racer) {
                            next_queue.push(next_racer);
                        }
                        continue;
                    }

                    // finished cheating
                    if racer.budget == 1 || (next_cost == 0 && racer.budget > 1) {
                        let mut next_cheat_path_keys = HashSet::new();

                        let key = (
                            racer.cheat_start.0,
                            racer.cheat_start.1,
                            next_pos.0,
                            next_pos.1,
                        );
                        if let Some(path_id) = paths.get(&key) {
                            next_cheat_path_keys.insert(*path_id);
                        } else {
                            path_count += 1;
                            paths.insert(key, path_count);
                            next_cheat_path_keys.insert(path_count);
                        }

                        let next_racer = Racer {
                            pos: next_pos,
                            budget: 0,
                            cheat_start: (u8::MAX, u8::MAX),
                            cheat_path_keys: next_cheat_path_keys,
                        };
                        if !next_queue.contains(&next_racer) {
                            next_queue.push(next_racer);
                        }
                        continue;
                    }

                    // in the middle of cheating
                    if racer.budget > 1 && next_cost > 0 {
                        let next_racer = Racer {
                            pos: next_pos,
                            budget: racer.budget - 1,
                            cheat_start: racer.cheat_start,
                            cheat_path_keys: HashSet::new(),
                        };
                        if !next_queue.contains(&next_racer) {
                            next_queue.push(next_racer);
                        }

                        continue;
                    }

                    let next_racer = Racer {
                        pos: next_pos,
                        budget: 0,
                        cheat_start: (u8::MAX, u8::MAX),
                        cheat_path_keys: racer.cheat_path_keys.clone(),
                    };

                    // try find an existing task in the queue to combine with
                    // instead of creating a new task
                    let mut found = false;
                    for racer_mut in next_queue.iter_mut() {
                        if racer_mut.budget == 0 && racer_mut.pos == next_racer.pos {
                            found = true;
                            racer_mut
                                .cheat_path_keys
                                .extend(next_racer.cheat_path_keys.clone());
                            break;
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

        eligible_path_keys.len() as u32
    }
}

fn main() {
    let input = include_str!("../example1.txt");
    let maze = Maze::new(input);
    let fastest_no_cheat = maze.fastest_traversal(0);
    dbg!(fastest_no_cheat);
    dbg!(maze.cheat_paths(2, fastest_no_cheat.saturating_sub(2)));
    //dbg!(maze.cheat_paths(5, fastest_no_cheat.saturating_sub(100)));
}
