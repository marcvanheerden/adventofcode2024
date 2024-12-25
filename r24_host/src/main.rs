use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq, Eq)]
enum Op {
    And,
    Or,
    Xor,
}

#[derive(Debug)]
struct Machine {
    vars: HashMap<[char; 3], bool>,
    code: Vec<(Op, [[char; 3]; 3])>,
}

impl Machine {
    fn new(input: &str) -> Self {
        let section_split = input.split_once("\n\n").unwrap();

        let vars = section_split
            .0
            .lines()
            .map(|line| {
                let line_split = line.split_once(": ").unwrap();
                (Self::name_to_arr(line_split.0), line_split.1 == "1")
            })
            .collect();

        let code = section_split
            .1
            .lines()
            .map(|l| {
                let (left, right) = l.split_once(" -> ").unwrap();
                let tokens: Vec<&str> = left.split(' ').collect();
                let arg1 = Self::name_to_arr(tokens[0]);
                let arg2 = Self::name_to_arr(tokens[2]);
                let arg3 = Self::name_to_arr(right);
                match tokens[1] {
                    "AND" => (Op::And, [arg1, arg2, arg3]),
                    "OR" => (Op::Or, [arg1, arg2, arg3]),
                    "XOR" => (Op::Xor, [arg1, arg2, arg3]),
                    _ => unreachable!(),
                }
            })
            .rev()
            .collect();

        Self { vars, code }
    }

    fn name_to_arr(name: &str) -> [char; 3] {
        let mut chars = name.chars();
        [
            chars.next().unwrap(),
            chars.next().unwrap(),
            chars.next().unwrap(),
        ]
    }

    fn run(&mut self) {
        let mut waiting = VecDeque::new();

        while !self.code.is_empty() || !waiting.is_empty() {
            let (op, [arg1, arg2, dest]) = if self.code.is_empty() {
                waiting.pop_front().unwrap()
            } else {
                self.code.pop().unwrap()
            };

            let val1 = self.vars.get(&arg1);
            let val2 = self.vars.get(&arg2);

            if val1.is_some() && val2.is_some() {
                let val1 = *val1.unwrap();
                let val2 = *val2.unwrap();
                let answer = match op {
                    Op::And => val1 & val2,
                    Op::Or => val1 | val2,
                    Op::Xor => val1 ^ val2,
                };

                self.vars
                    .entry(dest)
                    .and_modify(|v| *v = answer)
                    .or_insert(answer);
            } else {
                waiting.push_back((op, [arg1, arg2, dest]));
            }
        }
    }

    fn output(&self) -> usize {
        let mut zvars: Vec<_> = self.vars.iter().filter(|(k, _v)| k[0] == 'z').collect();
        zvars.sort_by_key(|(k, _v)| **k);
        dbg!(&zvars);
        let str_output: String = zvars
            .into_iter()
            .rev()
            .map(|(_k, v)| if *v { '1' } else { '0' })
            .collect();

        usize::from_str_radix(&str_output, 2).unwrap()
    }

    fn swap_dest(&mut self, dest1: [char; 3], dest2: [char; 3]) {
        let loc1 = self.code.iter().position(|c| c.1[2] == dest1).unwrap();
        let loc2 = self.code.iter().position(|c| c.1[2] == dest2).unwrap();

        self.code[loc1].1[2] = dest2;
        self.code[loc2].1[2] = dest1;
    }
}

fn part2(mut machine: Machine) {
    machine.swap_dest(['s', 'w', 't'], ['z', '0', '7']);
    machine.swap_dest(['p', 'q', 'c'], ['z', '1', '3']);
    machine.swap_dest(['r', 'j', 'm'], ['w', 's', 'v']);
    machine.swap_dest(['b', 'g', 's'], ['z', '3', '1']);

    let mut swaps = vec![
        ['s', 'w', 't'],
        ['p', 'q', 'c'],
        ['r', 'j', 'm'],
        ['b', 'g', 's'],
        ['z', '0', '7'],
        ['z', '1', '3'],
        ['w', 's', 'v'],
        ['z', '3', '1'],
    ];

    swaps.sort_unstable();
    println!();
    for dest in swaps.into_iter() {
        print!("{}{}{},", dest[0], dest[1], dest[2]);
    }
    println!();

    let mut inter_carry_bit: [char; 3] = [' '; 3];
    for bit_no in 0..=44 {
        let xbit: [char; 3] = Machine::name_to_arr(&format!("x{:0>2}", bit_no));
        let ybit: [char; 3] = Machine::name_to_arr(&format!("y{:0>2}", bit_no));
        let zbit: [char; 3] = Machine::name_to_arr(&format!("z{:0>2}", bit_no));

        let sum_bit = machine
            .code
            .iter()
            .find(|(op, args)| {
                *op == Op::Xor && args[0..=1].contains(&xbit) && args[0..=1].contains(&ybit)
            })
            .unwrap();

        let carry_bit1 = machine
            .code
            .iter()
            .find(|(op, args)| {
                *op == Op::And && args[0..=1].contains(&xbit) && args[0..=1].contains(&ybit)
            })
            .unwrap();

        if bit_no == 0 && sum_bit.1[2] == zbit {
            inter_carry_bit = carry_bit1.1[2];
            continue;
        }

        let bit_write = machine.code.iter().find(|(op, args)| {
            *op == Op::Xor
                && args[0..=1].contains(&inter_carry_bit.clone())
                && args[0..=1].contains(&sum_bit.1[2])
        });

        if bit_write.is_none() {
            dbg!(&inter_carry_bit);
            dbg!(&sum_bit.1[2]);
            dbg!(machine.code.iter().find(|(op, args)| {
                *op == Op::Xor
                    && (args[0..=1].contains(&inter_carry_bit.clone())
                        || args[0..=1].contains(&sum_bit.1[2]))
            }));
        }

        assert_eq!(bit_write.unwrap().1[2], zbit);

        let carry_bit2 = machine
            .code
            .iter()
            .find(|(op, args)| {
                *op == Op::And
                    && args[0..=1].contains(&inter_carry_bit.clone())
                    && args[0..=1].contains(&sum_bit.1[2])
            })
            .unwrap();

        let next_carry = machine
            .code
            .iter()
            .find(|(op, args)| {
                *op == Op::Or
                    && args[0..=1].contains(&carry_bit1.1[2])
                    && args[0..=1].contains(&carry_bit2.1[2])
            })
            .unwrap();

        inter_carry_bit = next_carry.1[2];
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut machine = Machine::new(input);
    machine.run();
    dbg!(machine.output());

    let machine = Machine::new(input);
    part2(machine);
}
