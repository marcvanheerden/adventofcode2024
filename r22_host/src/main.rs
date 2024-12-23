use rayon::prelude::*;
use std::collections::VecDeque;

const PRUNE: isize = 16777216;
const RUNLEN: isize = 2000;

fn evolve(secret_num: isize) -> isize {
    let step1 = ((secret_num * 64) ^ secret_num) % PRUNE;
    let step2 = ((step1 / 32) ^ step1) % PRUNE;
    ((step2 * 2048) ^ step2) % PRUNE
}

fn part2(input: &str) -> isize {
    let mut patterns: Vec<[isize; 4]> = Vec::new();
    for diff1 in -9..=9 {
        for diff2 in -9..=9 {
            for diff3 in -9..=9 {
                for diff4 in -9..=9 {
                    patterns.push([diff1, diff2, diff3, diff4])
                }
            }
        }
    }

    patterns.retain(|pattern| {
        (-9..=9isize).contains(&pattern.iter().cloned().sum())
            && (-9..=9isize).contains(&pattern.iter().take(3).cloned().sum())
            && (-9..=9isize).contains(&pattern.iter().take(2).cloned().sum())
            && (-9..=9isize).contains(&pattern.iter().skip(2).take(2).cloned().sum())
            && (-9..=9isize).contains(&pattern.iter().skip(1).take(3).cloned().sum())
            && (-9..=9isize).contains(&pattern.iter().skip(1).take(2).cloned().sum())
    });

    let starting_secrets: Vec<isize> = input.lines().map(|s| s.parse().unwrap()).collect();

    patterns
        .par_iter()
        .map(|pattern| {
            starting_secrets
                .iter()
                .map(|start| {
                    let mut num = *start;
                    let mut next_num;
                    let mut delta_run = VecDeque::new();
                    for _ in 0..RUNLEN {
                        next_num = evolve(num);
                        if delta_run.len() >= 4 {
                            let _ = delta_run.pop_front();
                        }
                        delta_run.push_back((next_num % 10) - (num % 10));

                        if delta_run.len() == 4
                            && delta_run.iter().zip(pattern).all(|(a, b)| a == b)
                        {
                            return next_num % 10;
                        }
                        num = next_num;
                    }
                    0
                })
                .sum::<isize>()
        })
        .max()
        .unwrap()
}

fn main() {
    let input = include_str!("../input.txt");

    let part1 = input
        .lines()
        .map(|s| {
            let mut num = s.parse().unwrap();
            for _ in 0..RUNLEN {
                num = evolve(num);
            }
            num
        })
        .sum::<isize>();

    dbg!(part1);

    dbg!(part2(input));
}
