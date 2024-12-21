use std::collections::{HashMap, VecDeque};

type KeySequence = Vec<u8>;

const NUMPAD: &str = "789
456
123
X0A";

const DIRPAD: &str = "X^A
<v>";

#[derive(Debug)]
struct InputDevice {
    layout: Vec<(u8, u8, u8)>,
    cache: HashMap<(u8, u8), Vec<Vec<u8>>>,
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

    fn paths(&self, start: (u8, u8), stop: (u8, u8)) -> Vec<KeySequence> {
        let mut queue = VecDeque::new();
        queue.push_front(vec![(start.0, start.1, b'x')]);

        let mut paths = Vec::new();
        while !queue.is_empty() && paths.is_empty() {
            for _ in 0..queue.len() {
                let task = queue.pop_front().unwrap();
                let task_current = task.last().unwrap();

                if (task_current.0, task_current.1) == stop {
                    let mut moves: Vec<_> =
                        task.into_iter().skip(1).map(|(_y, _x, dir)| dir).collect();
                    moves.push(b'A');
                    paths.push(moves);
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

    fn ways_to_move(&mut self, from_key: u8, to_key: u8) -> Vec<KeySequence> {
        if let Some(output) = self.cache.get(&(from_key, to_key)) {
            return output.to_vec();
        }

        let from_loc = self
            .find(from_key)
            .expect("looking for a key that doesn't exist");
        let to_loc = self
            .find(to_key)
            .expect("looking for a key that doesn't exist");

        let ways = self.paths(from_loc, to_loc);
        self.cache.insert((from_key, to_key), ways.clone());
        ways
    }
}

fn meta_step(device: &mut InputDevice, sequence: KeySequence) -> Vec<KeySequence> {
    let mut full_sequence = vec![b'A'];
    full_sequence.extend(sequence);

    let sequence_parts: Vec<_> = full_sequence
        .windows(2)
        .map(|w| device.ways_to_move(w[0], w[1]))
        .collect();

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
                        s_clone.extend(p);
                        s_clone
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
    }

    sequences
}

fn part1(passcode: &str) -> usize {
    let mut numpad = InputDevice::new(NUMPAD);
    let mut dirpad = InputDevice::new(DIRPAD);

    let initial_sequence: Vec<_> = passcode.chars().map(|c| c as u8).collect();

    let mut sequences = meta_step(&mut numpad, initial_sequence);

    sequences = sequences
        .into_iter()
        .flat_map(|s| meta_step(&mut dirpad, s))
        .collect();

    sequences = sequences
        .into_iter()
        .flat_map(|s| meta_step(&mut dirpad, s))
        .collect();

    let min_key_seq = sequences.into_iter().map(|s| s.len()).min().unwrap();
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
        for key in seq.iter() {
            print!("{}", *key as char);
        }
        println!();
    }
    println!("---------------------------------------------");
}

fn main() {
    let input = include_str!("../input.txt");
    dbg!(input.lines().map(part1).sum::<usize>());
}

// Assumptions: adding additional moves will never be more efficient
