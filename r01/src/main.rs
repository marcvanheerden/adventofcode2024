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

const MAXLEN: usize = 1001;

fn process_inputs(input: &str) -> (Vec<i32, MAXLEN>, Vec<i32, MAXLEN>) {
    let mut vec1: Vec<i32, MAXLEN> = Vec::new();
    let mut vec2: Vec<i32, MAXLEN> = Vec::new();

    for line in input.lines() {
        let mut split = line.split("   ");
        let left = split
            .next()
            .expect("expected two tab seperated number columns");
        let right = split
            .next()
            .expect("expected two tab seperated number columns");

        let _ = vec1.push(left.parse().unwrap());
        let _ = vec2.push(right.parse().unwrap());
    }

    (vec1, vec2)
}

fn part1(mut vec1: Vec<i32, MAXLEN>, mut vec2: Vec<i32, MAXLEN>) -> i32 {
    vec1.sort_unstable();
    vec2.sort_unstable();

    vec1.into_iter()
        .zip(vec2)
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>()
}

fn part2(vec1: &Vec<i32, MAXLEN>, vec2: &Vec<i32, MAXLEN>) -> i32 {
    vec1.iter()
        .map(|&i| vec2.iter().filter(|&x| *x == i).count() as i32 * i)
        .sum()
}

#[entry]
fn main() -> ! {
    info!("Program start");

    let inputs = include_str!("../input.txt");
    let (vec1, vec2) = process_inputs(inputs);

    let ans2 = part2(&vec1, &vec2);
    let ans1 = part1(vec1, vec2);
    info!("calculation finished");

    // forcing the compiler to keeps these alive so I can view them
    // with the debugger
    black_box(ans1);
    black_box(ans2);
    loop {}
}
