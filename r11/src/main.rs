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

const MAX_STONES: usize = 1000;

#[derive(Clone)]
struct Stone {
    num: u64,
    count: u64,
}

fn split_digits(num: u64) -> Option<(u64, u64)> {
    let mut digits: u32 = 0;

    for idx in 1..30u32 {
        if num < 10u64.pow(idx) {
            digits = idx;
            break;
        }
    }

    if digits % 2 == 0 {
        let front = num / 10u64.pow(digits / 2);
        let back = num - (front * 10u64.pow(digits / 2));
        return Some((front, back));
    }

    None
}

fn part1(input: &str, blinks: u8) -> u64 {
    let mut stones: Vec<Stone, MAX_STONES> = input
        .lines()
        .next()
        .unwrap()
        .split(' ')
        .map(|n| Stone {
            num: n.parse().unwrap(),
            count: 1,
        })
        .collect();

    for _blink in 0..blinks {
        for idx in 0..stones.len() {
            if stones[idx].num == 0 {
                stones[idx].num = 1;
                continue;
            }

            if let Some((front, back)) = split_digits(stones[idx].num) {
                let mut found_front = None;
                let mut found_back = None;

                for idx2 in 0..stones.len() {
                    if stones[idx2].num == front {
                        found_front = Some(idx2);
                        break;
                    }
                    if stones[idx2].num == back {
                        found_back = Some(idx2);
                        break;
                    }
                }

                if let Some(idx2) = found_front {
                    stones[idx2].count += stones[idx].count;
                    stones[idx].num = back;
                } else if let Some(idx2) = found_back {
                    stones[idx2].count += stones[idx].count;
                    stones[idx].num = front;
                } else {
                    stones[idx].num = front;
                    let _ = stones.push(Stone {
                        num: back,
                        count: stones[idx].count,
                    });
                }
            } else {
                stones[idx].num *= 2024;
            }
        }
    }

    stones.into_iter().map(|s| s.count).sum()
}

#[entry]
fn main() -> ! {
    info!("Program start");

    let inputs = include_str!("../example1.txt");
    let ans1 = part1(inputs, 25);
    info!("calculation finished");

    // forcing the compiler to keeps these alive so I can view them
    // with the debugger
    black_box(ans1);
    loop {}
}
