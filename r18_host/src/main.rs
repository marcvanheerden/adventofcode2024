use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone)]
struct MemoryBank {
    corrupted_bits: HashSet<(u8, u8)>,
    incoming: Vec<(u8, u8)>,
    end: (u8, u8),
}

impl MemoryBank {
    fn new(input: &str, end: (u8, u8)) -> Self {
        let incoming = input
            .lines()
            .rev()
            .map(|s| {
                let (y, x) = s.split_once(',').unwrap();
                (y.parse().unwrap(), x.parse().unwrap())
            })
            .collect();

        let corrupted_bits = HashSet::new();

        MemoryBank {
            corrupted_bits,
            incoming,
            end,
        }
    }

    fn next_fall(&mut self) {
        if let Some(bit) = self.incoming.pop() {
            self.corrupted_bits.insert(bit);
        }
    }

    fn many_fall(&mut self, bits: u32) {
        for _ in 0..bits {
            self.next_fall();
        }
    }

    fn next_steps(&self, y: u8, x: u8) -> Vec<(u8, u8)> {
        let mut output = Vec::with_capacity(4);
        if (y > 0) && !self.corrupted_bits.contains(&(y - 1, x)) {
            output.push((y - 1, x));
        }

        if (y < self.end.0) && !self.corrupted_bits.contains(&(y + 1, x)) {
            output.push((y + 1, x));
        }

        if (x > 0) && !self.corrupted_bits.contains(&(y, x - 1)) {
            output.push((y, x - 1));
        }

        if (x < self.end.1) && !self.corrupted_bits.contains(&(y, x + 1)) {
            output.push((y, x + 1));
        }

        output
    }
}

fn traverse(mut mb: MemoryBank, initial_drops: u32) -> Option<u32> {
    mb.many_fall(initial_drops);

    let mut queue = VecDeque::new();
    queue.push_front((0, 0));

    let mut history = HashSet::new();
    history.insert((0, 0));

    let mut steps = 0;
    while !queue.is_empty() {
        for _ in 0..queue.len() {
            let (y, x) = queue.pop_front().unwrap();

            if (y, x) == mb.end {
                return Some(steps);
            }

            for step in mb.next_steps(y, x) {
                if !history.contains(&step) {
                    history.insert(step);
                    queue.push_back(step);
                }
            }
        }
        steps += 1;
    }

    None
}

fn main() {
    let input = include_str!("../input.txt");
    let mb = MemoryBank::new(input, (70, 70));

    dbg!(traverse(mb.clone(), 1024));

    for initial_drops in 1024..input.lines().count() {
        if traverse(mb.clone(), initial_drops as u32).is_none() {
            dbg!(mb.incoming.iter().rev().nth(initial_drops - 1));
            break;
        }
    }
}
