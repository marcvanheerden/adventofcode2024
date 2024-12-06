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

const MAX_PAGES: usize = 100;
const MAX_DEPS: usize = 100;
const MAX_UPDATES: usize = 30;

type DepList = Vec<Vec<u8, MAX_DEPS>, MAX_PAGES>;

#[derive(PartialEq)]
enum Mode {
    FixUpdates,
    DontFix,
}

fn check_updates(input: &str, mode: Mode) -> u32 {
    let mut deplist: DepList = (0..MAX_PAGES).map(|_| Vec::new()).collect();
    let mut update: Vec<u8, MAX_UPDATES> = Vec::new();
    let mut first_section = true;
    let mut total = 0;
    let mut correct_order = true;

    for line in input.lines() {
        if line.trim().is_empty() {
            first_section = false;
            continue;
        }

        if first_section {
            let mut split = line.splitn(2, '|');
            let dep: u8 = split.next().unwrap().parse().unwrap();
            let page: usize = split.next().unwrap().parse().unwrap();
            let _ = deplist[page].push(dep);
        } else {
            for page_str in line.split(',') {
                let page: u8 = page_str.parse().unwrap();

                for prev_page in update.iter() {
                    if deplist[*prev_page as usize].contains(&page) {
                        correct_order = false;
                    }
                }

                let _ = update.push(page);
            }

            if (mode == Mode::DontFix) && correct_order {
                total += update[update.len() / 2] as u32;
            }

            if (mode == Mode::FixUpdates) && !correct_order {
                fix(&mut update, &deplist);
                total += update[update.len() / 2] as u32;
            }

            update.clear();
            correct_order = true;
        }
    }

    total
}

fn fix(update: &mut Vec<u8, MAX_UPDATES>, deplist: &DepList) {
    loop {
        for idx in 1..update.len() {
            for prev_idx in 0..idx {
                if deplist[update[prev_idx] as usize].contains(&update[idx]) {
                    update.swap(idx, prev_idx);
                    continue;
                }
            }
        }

        break;
    }
}

#[entry]
fn main() -> ! {
    info!("Program start");

    let inputs = include_str!("../input.txt");

    let ans1 = check_updates(inputs, Mode::DontFix);
    let ans2 = check_updates(inputs, Mode::FixUpdates);
    info!("calculation finished");

    // forcing the compiler to keeps these alive so I can view them
    // with the debugger
    black_box(ans1);
    black_box(ans2);
    loop {}
}
