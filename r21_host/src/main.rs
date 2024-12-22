use std::collections::{HashMap, VecDeque};

const NUMPAD: &str = "789
456
123
X0A";

const DIRPAD: &str = "X^A
<v>";

#[derive(Debug)]
struct InputDevice {
    layout: Vec<(u8, u8, u8)>,
    cache: HashMap<(u8, u8), Vec<KeySequence>>,
}

impl InputDevice {
    fn new(input: &str) -> Self {
        let mut layout = Vec::new();
        for (row, line) in input.lines().enumerate() {
            for (col, chr) in line.chars().enumerate() {
                layout.push((row as u8, col as u8, chr as u8));
            }
        }

        Self {
            layout,
            cache: HashMap::new(),
        }
    }

    fn can_go_here(&self, pos: &(u8, u8)) -> bool {
        self.layout
            .iter()
            .any(|(y, x, c)| (*y, *x) == *pos && *c != b'X')
    }

    fn find(&self, key: u8) -> Option<(u8, u8)> {
        self.layout
            .iter()
            .filter_map(|(y, x, c)| if *c == key { Some((*y, *x)) } else { None })
            .next()
    }

    fn next_steps(&self, pos: (u8, u8)) -> Vec<(u8, u8, u8)> {
        let mut output = Vec::with_capacity(4);

        if pos.0 > 0 {
            let up = (pos.0 - 1, pos.1, b'^');
            if self.can_go_here(&(up.0, up.1)) {
                output.push(up);
            }
        }

        if pos.1 > 0 {
            let left = (pos.0, pos.1 - 1, b'<');
            if self.can_go_here(&(left.0, left.1)) {
                output.push(left);
            }
        }

        let down = (pos.0 + 1, pos.1, b'v');
        if self.can_go_here(&(down.0, down.1)) {
            output.push(down);
        }

        let right = (pos.0, pos.1 + 1, b'>');
        if self.can_go_here(&(right.0, right.1)) {
            output.push(right);
        }

        output
    }

    fn paths(&self, start: (u8, u8), stop: (u8, u8), count: usize) -> Vec<KeySequence> {
        let mut queue = VecDeque::new();
        queue.push_front(vec![(start.0, start.1, b'x')]);

        let mut paths: Vec<KeySequence> = Vec::new();
        while !queue.is_empty() && paths.is_empty() {
            for _ in 0..queue.len() {
                let task = queue.pop_front().unwrap();
                let task_current = task.last().unwrap();

                if (task_current.0, task_current.1) == stop {
                    let mut moves: Vec<_> =
                        task.into_iter().skip(1).map(|(_y, _x, dir)| dir).collect();
                    moves.push(b'A');
                    let moves = moves
                        .windows(2)
                        .map(|w| KeyTransition {
                            from: w[0],
                            to: w[1],
                            count,
                        })
                        .collect();
                    paths.push(KeySequence { trs: moves });
                    continue;
                }

                for next in self.next_steps((task_current.0, task_current.1)) {
                    let mut next_task = task.clone();
                    next_task.push(next);
                    queue.push_back(next_task);
                }
            }
        }

        paths
    }

    fn ways_to_move(&mut self, from_key: u8, to_key: u8, count: usize) -> Vec<KeySequence> {
        if let Some(output) = self.cache.get(&(from_key, to_key)) {
            return output.clone();
        }

        let from_loc = self
            .find(from_key)
            .expect("looking for a key that doesn't exist");
        let to_loc = self
            .find(to_key)
            .expect("looking for a key that doesn't exist");

        let ways = self.paths(from_loc, to_loc, count);
        self.cache.insert((from_key, to_key), ways.clone());
        ways
    }
}

#[derive(Debug, Clone, PartialEq)]
struct KeyTransition {
    from: u8,
    to: u8,
    count: usize,
}

#[derive(Debug, Clone, PartialEq)]
struct KeySequence {
    trs: Vec<KeyTransition>,
}

impl KeySequence {
    fn compress(&mut self) {
        // combine same key transitions and sum their counts
        // as long as the first and the last are unmoved then its fine
        'outer: loop {
            for early_idx in 0..(self.trs.len() - 1) {
                for later_idx in (early_idx + 1)..(self.trs.len() - 1) {
                    if self.trs[early_idx].from == self.trs[later_idx].from
                        && self.trs[early_idx].to == self.trs[later_idx].to
                    {
                        self.trs[early_idx].count += self.trs[later_idx].count;
                        self.trs.remove(later_idx);
                        continue 'outer;
                    }
                }
            }
            break;
        }
    }

    fn total(&self) -> usize {
        self.trs.iter().map(|kt| kt.count).sum::<usize>() + 1
    }
}

fn meta_step(device: &mut InputDevice, sequence: KeySequence) -> Vec<KeySequence> {
    // add the starting transition from A to first key press
    let starting_key = sequence.trs.first().unwrap().from;
    let mut full_sequence = KeySequence {
        trs: vec![KeyTransition {
            from: b'A',
            to: starting_key,
            count: 1,
        }],
    };
    full_sequence.trs.extend(sequence.trs);

    // get the different key combinations for each transition
    let sequence_parts: Vec<Vec<KeySequence>> = full_sequence
        .trs
        .into_iter()
        .map(|tr| device.ways_to_move(tr.from, tr.to, tr.count))
        .collect();

    // combine all the key combinations for the transitions
    let mut sequences: Vec<KeySequence> = Vec::new();
    for part in sequence_parts.into_iter() {
        if sequences.is_empty() {
            sequences = part;
            continue;
        }

        sequences = sequences
            .into_iter()
            .flat_map(|s| {
                part.iter()
                    .map(|p| {
                        let mut s_clone = s.clone();
                        let mut next_to = b'A';

                        if let Some(next_trs) = p.trs.first() {
                            next_to = next_trs.from;
                        }

                        let joiner = KeyTransition {
                            from: s.trs.last().unwrap().to,
                            to: next_to,
                            count: 1,
                        };
                        s_clone.trs.push(joiner);
                        s_clone.trs.extend(p.trs.clone());
                        s_clone
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
    }

    sequences
}

fn propagate(passcode: &str, robots: usize) -> usize {
    let mut numpad = InputDevice::new(NUMPAD);
    let mut dirpad = InputDevice::new(DIRPAD);

    let initial_sequence: Vec<_> = passcode.chars().map(|c| c as u8).collect();
    let initial_sequence = initial_sequence
        .windows(2)
        .map(|w| KeyTransition {
            from: w[0],
            to: w[1],
            count: 1,
        })
        .collect();
    let initial_sequence = KeySequence {
        trs: initial_sequence,
    };

    let mut sequences = meta_step(&mut numpad, initial_sequence);

    easy_display(&sequences);
    for _ in 0..robots {
        sequences = sequences
            .into_iter()
            .flat_map(|s| meta_step(&mut dirpad, s))
            .collect();
        sequences.iter_mut().for_each(|s| s.compress());
        easy_display(&sequences);
    }

    let min_key_seq = sequences.into_iter().map(|s| s.total()).min().unwrap();
    let number_parts: usize = passcode
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap();

    min_key_seq * number_parts
}

fn easy_display(ks: &[KeySequence]) {
    println!("---------------------------------------------");
    for seq in ks.iter() {
        for key in seq.trs.iter() {
            print!("{}{}x{} ", key.from as char, key.to as char, key.count);
        }
        println!();
    }
    println!("---------------------------------------------");
}

fn main() {
    let input = include_str!("../example1.txt");
    dbg!(input
        .lines()
        .take(1)
        .map(|s| propagate(s, 0))
        .sum::<usize>());
}

// Assumptions: adding additional moves will never be more efficient
// TODOs:
// Order all the middle trs and then deduplicate
// Debug thist thing
