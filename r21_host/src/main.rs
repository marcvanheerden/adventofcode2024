use std::collections::{HashMap, VecDeque};
use std::fmt;

const NUMPAD: &str = "789
456
123
X0A";

const DIRPAD: &str = "X^A
<v>";

#[derive(Debug)]
struct InputDevice {
    layout: Vec<(u8, u8, u8)>,
    cache: HashMap<(u8, u8, u8), Vec<Vec<KeyTransition>>>,
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

    fn paths(&self, start: (u8, u8), stop: (u8, u8), depth: u8) -> Vec<Vec<KeyTransition>> {
        //if start == stop {
        //    return vec![vec![KeyTransition {
        //        from: b'A',
        //        to: b'A',
        //        depth: depth + 1,
        //    }]];
        //}

        let mut queue = VecDeque::new();
        queue.push_front(vec![(start.0, start.1, b'x')]);

        let mut paths: Vec<Vec<KeyTransition>> = Vec::new();
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
                            depth: depth + 1,
                        })
                        .collect();
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

    fn ways_to_move(&mut self, from_key: u8, to_key: u8, depth: u8) -> Vec<Vec<KeyTransition>> {
        if let Some(output) = self.cache.get(&(from_key, to_key, depth)) {
            return output.clone();
        }
        let from_loc = self
            .find(from_key)
            .expect("looking for a key that doesn't exist");
        let to_loc = self
            .find(to_key)
            .expect("looking for a key that doesn't exist");

        let ways = self.paths(from_loc, to_loc, depth);
        self.cache.insert((from_key, to_key, depth), ways.clone());
        ways
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct KeyTransition {
    from: u8,
    to: u8,
    depth: u8,
}

impl fmt::Debug for KeyTransition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&(self.from as char))
            .field(&(self.to as char))
            .field(&self.depth)
            .finish()
    }
}

fn easy_display(ks: &[Vec<KeyTransition>]) {
    println!("---------------------------------------------");
    for seq in ks.iter() {
        for key in seq.iter() {
            print!("{}{}x{} ", key.from as char, key.to as char, key.depth);
        }
        println!();
    }
    println!("---------------------------------------------");
}

fn propagate(
    input: &str,
    max_depth: u8,
    numpad: &mut InputDevice,
    dirpad: &mut InputDevice,
) -> usize {
    let chars: Vec<_> = input.chars().map(|c| c as u8).collect();
    let mut all_chars = vec![b'A'];
    all_chars.extend(chars);
    let kts: Vec<_> = all_chars
        .windows(2)
        .map(|w| KeyTransition {
            from: w[0],
            to: w[1],
            depth: 0,
        })
        .collect();

    let mut cache = HashMap::new();

    kts.into_iter()
        .map(|kt| plummit(kt, max_depth, numpad, dirpad, &mut cache))
        .sum()
}

fn plummit(
    kt: KeyTransition,
    max_depth: u8,
    numpad: &mut InputDevice,
    dirpad: &mut InputDevice,
    cache: &mut HashMap<KeyTransition, usize>,
) -> usize {
    if let Some(output) = cache.get(&kt) {
        return *output;
    }

    if kt.depth > max_depth {
        return 1;
    }

    let mut wtm = if kt.depth == 0 {
        numpad.ways_to_move(kt.from, kt.to, kt.depth)
    } else {
        dirpad.ways_to_move(kt.from, kt.to, kt.depth)
    };

    wtm.iter_mut().for_each(|v| {
        let to = if let Some(fkt) = v.first() {
            fkt.from
        } else {
            b'A'
        };
        let mut newv = vec![KeyTransition {
            from: b'A',
            to,
            depth: kt.depth + 1,
        }];
        newv.extend(v.clone());
        *v = newv;
    });

    let output = wtm
        .iter()
        .map(|vkt| {
            vkt.iter()
                .map(|kt| plummit(kt.clone(), max_depth, numpad, dirpad, cache))
                .sum::<usize>()
        })
        .min()
        .unwrap();

    cache.insert(kt, output);

    output
}

fn main() {
    let mut numpad = InputDevice::new(NUMPAD);
    let mut dirpad = InputDevice::new(DIRPAD);

    let input = include_str!("../input.txt");
    let part1 = input
        .lines()
        .map(|s| {
            let number_part: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
            let min_length = propagate(s, 2, &mut numpad, &mut dirpad);
            min_length * number_part.parse::<usize>().unwrap()
        })
        .sum::<usize>();

    dbg!(part1);

    let part2 = input
        .lines()
        .map(|s| {
            let number_part: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
            let min_length = propagate(s, 25, &mut numpad, &mut dirpad);
            min_length * number_part.parse::<usize>().unwrap()
        })
        .sum::<usize>();

    dbg!(part2);
}
