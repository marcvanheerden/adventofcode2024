use std::collections::VecDeque;

fn match_pattern(pattern: &str, towels: &[Vec<u8>]) -> u64 {
    let mut ways_to_solve_this_line = 0;
    let mut queue: VecDeque<(Vec<u8>, u64)> = VecDeque::new();
    let characters = pattern.chars().rev().map(|c| c as u8).collect();
    queue.push_front((characters, 1));

    while let Some((stub, count)) = queue.pop_front() {
        if stub.is_empty() {
            ways_to_solve_this_line += count;
        }

        for towel in towels.iter() {
            if towel.len() <= stub.len()
                && towel
                    .iter()
                    .zip(stub.iter().rev())
                    .all(|(c1, c2)| *c1 == *c2)
            {
                let mut next_stub = stub.clone();
                for _ in 0..towel.len() {
                    let _ = next_stub.pop();
                }

                let mut found = false;
                for (other_stub, other_count) in queue.iter_mut() {
                    if *other_stub == next_stub {
                        *other_count += count;
                        found = true;
                        break;
                    }
                }

                if !found {
                    queue.push_back((next_stub, count));
                }
            }
        }
    }

    ways_to_solve_this_line
}

fn matchymatchy(input: &str) -> (u64, u64) {
    let mut lines = input.lines();

    let towels: Vec<Vec<u8>> = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|t| t.chars().map(|c| c as u8).collect())
        .collect();

    let mut possible_designs = 0;
    let mut possible_ways = 0;

    for line in lines.skip(1) {
        let possible_ways_of_matching = match_pattern(line, &towels);
        if possible_ways_of_matching > 0 {
            possible_designs += 1;
        }
        possible_ways += possible_ways_of_matching;
    }

    (possible_designs, possible_ways)
}

fn main() {
    let input = include_str!("../input.txt");
    dbg!(matchymatchy(input));
}
