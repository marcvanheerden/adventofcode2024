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

const MAX_LEVELS: usize = 10;

#[derive(PartialEq)]
enum Mode {
    Tolerant,
    Intolerant,
}

fn count_safe_levels(input: &str, mode: Mode) -> i32 {
    let mut safe_lines = 0;

    for line in input.lines() {
        let levels: Vec<i32, MAX_LEVELS> = line.split(' ').map(|s| s.parse().unwrap()).collect();

        if assess_safety(&levels) {
            safe_lines += 1;
            continue;
        }

        if mode == Mode::Tolerant {
            for to_remove in 0..levels.len() {
                let mut remaining_levels = levels.clone();
                remaining_levels.remove(to_remove);
                if assess_safety(&remaining_levels) {
                    safe_lines += 1;
                    break;
                }
            }
        }
    }

    safe_lines
}

fn assess_safety(levels: &Vec<i32, MAX_LEVELS>) -> bool {
    let diffs: Vec<_, { MAX_LEVELS - 1 }> = levels.windows(2).map(|w| w[0] - w[1]).collect();

    let max = diffs.iter().max().unwrap();
    let min = diffs.iter().min().unwrap();

    if (*min < 0) && (*max > 0) {
        return false;
    }

    if (*min == 0) || (*max == 0) {
        return false;
    }

    if (min.abs() > 3) || (max.abs() > 3) {
        return false;
    }

    true
}

#[entry]
fn main() -> ! {
    info!("Program start");

    let inputs = include_str!("../input.txt");

    let ans1 = count_safe_levels(inputs, Mode::Intolerant);
    let ans2 = count_safe_levels(inputs, Mode::Tolerant);
    info!("calculation finished");

    // forcing the compiler to keeps these alive so I can view them
    // with the debugger
    black_box(ans1);
    black_box(ans2);
    loop {}
}
