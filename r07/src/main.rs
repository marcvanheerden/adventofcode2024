#![no_std]
#![no_main]

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use rp_pico as bsp;

use core::hint::black_box;
use core::str;
use heapless::Vec;

const MAX_TERMS: usize = 30;

fn conc(pre: u64, post: u64) -> u64 {
    for expo in 1..20 {
        if post < 10_u64.pow(expo) {
            return pre * 10u64.pow(expo) + post;
        }
    }

    return 0;
}

fn can_calc(mut terms: Vec<u64, MAX_TERMS>, current: u64, target: u64, concat: bool) -> bool {
    if current > target {
        return false;
    }

    if let Some(next_term) = terms.pop() {
        let add_next = can_calc(terms.clone(), current + next_term, target, concat);
        let mul_next = can_calc(terms.clone(), current * next_term, target, concat);

        if concat {
            let concat_next = can_calc(terms, conc(current, next_term), target, concat);
            return add_next || mul_next || concat_next;
        } else {
            return add_next || mul_next;
        }
    }

    target == current
}

fn calibrate(input: &str) -> (u64, u64) {
    let mut part1 = 0u64;
    let mut part2 = 0u64;

    for line in input.lines() {
        let mut split1 = line.split(": ");
        let target: u64 = split1.next().unwrap().parse().unwrap();
        let mut terms: Vec<u64, MAX_TERMS> = split1
            .next()
            .unwrap()
            .split(' ')
            .map(|s| s.parse::<u64>().unwrap())
            .rev()
            .collect();

        let first_term = terms.pop().unwrap();
        if can_calc(terms.clone(), first_term, target, false) {
            part1 += target;
            part2 += target;
        } else if can_calc(terms, first_term, target, true) {
            part2 += target;
        }
    }

    (part1, part2)
}

#[entry]
fn main() -> ! {
    info!("Program start");

    let inputs = include_str!("../input.txt");

    let ans1 = calibrate(inputs);
    info!("calculation finished");

    // forcing the compiler to keeps these alive so I can view them
    // with the debugger
    black_box(ans1);
    loop {}
}
