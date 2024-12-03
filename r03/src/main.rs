#![no_std]
#![no_main]

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use rp_pico as bsp;

use core::hint::black_box;
use core::str;
use heapless::{String, Vec};

const MUL_PATTERN: &[char] = &[')', ' ', ',', ' ', '(', 'l', 'u', 'm'];
const MUL_PATTERN_LEN: usize = 8;
const MAX_DIGITS: usize = 3;
const START_PATTERN: &[char] = &[')', '(', 'o', 'd'];
const START_PATTERN_LEN: usize = 4;
const END_PATTERN: &[char] = &[')', '(', 't', '\'', 'n', 'o', 'd'];
const END_PATTERN_LEN: usize = 7;

#[derive(PartialEq)]
enum State {
    CollectingDigits,
    LookingForMul,
    NotLooking,
}

fn part1(input: &str) -> i32 {
    let mut state = State::LookingForMul;
    let mut total: i32 = 0;
    let mut product: i32 = 1;
    let mut digits: Vec<char, MAX_DIGITS> = Vec::new();
    let mut reset = false;

    let mut mul_chars: Vec<char, MUL_PATTERN_LEN> = Vec::from_slice(MUL_PATTERN).unwrap();

    for chr in input.chars() {
        if reset {
            mul_chars = Vec::from_slice(MUL_PATTERN).unwrap();
            digits = Vec::new();
            reset = false;
            product = 1;
        }

        let next_char = mul_chars.last().unwrap();
        match state {
            State::LookingForMul => {
                if *next_char == ' ' {
                    if chr.is_ascii_digit() {
                        state = State::CollectingDigits;
                        let _ = mul_chars.pop();
                        digits = Vec::new();
                        let _ = digits.push(chr);
                    } else {
                        reset = true;
                    }
                    continue;
                }

                if *next_char == chr {
                    match mul_chars.pop() {
                        Some(_) => continue,
                        None => defmt::unreachable!(),
                        // will be reached in CollectingDigits state
                    }
                } else {
                    reset = true;
                }
            }
            State::CollectingDigits => {
                if chr.is_ascii_digit() {
                    match digits.push(chr) {
                        Ok(_) => {}
                        Err(_) => {
                            reset = true;
                        }
                    }
                } else if chr == *next_char {
                    let all_digits: String<MAX_DIGITS> = digits.iter().collect();
                    product *= all_digits.parse::<i32>().unwrap();
                    state = State::LookingForMul;
                    let _ = mul_chars.pop();

                    if mul_chars.is_empty() {
                        total += product;
                        reset = true;
                    }
                    continue;
                } else {
                    state = State::LookingForMul;
                    reset = true;
                }
            }
            State::NotLooking => continue,
        }
    }

    total
}

fn part2(input: &str) -> i32 {
    let mut state = State::LookingForMul;
    let mut total: i32 = 0;
    let mut product: i32 = 1;
    let mut digits: Vec<char, MAX_DIGITS> = Vec::new();
    let mut reset = false;
    let mut reset_stop_start = false;

    let mut mul_chars: Vec<char, MUL_PATTERN_LEN> = Vec::from_slice(MUL_PATTERN).unwrap();
    let mut start_chars: Vec<char, START_PATTERN_LEN> = Vec::from_slice(START_PATTERN).unwrap();
    let mut end_chars: Vec<char, END_PATTERN_LEN> = Vec::from_slice(END_PATTERN).unwrap();

    for chr in input.chars() {
        if reset {
            mul_chars = Vec::from_slice(MUL_PATTERN).unwrap();
            digits = Vec::new();
            product = 1;
            reset = false;
        }

        if reset_stop_start {
            start_chars = Vec::from_slice(START_PATTERN).unwrap();
            end_chars = Vec::from_slice(END_PATTERN).unwrap();
            reset_stop_start = false;
        }

        let next_char = mul_chars.last().unwrap();
        let next_start_char = start_chars.last().unwrap();
        let next_end_char = end_chars.last().unwrap();

        match state {
            State::LookingForMul => {
                if *next_end_char == chr {
                    let _ = end_chars.pop();

                    if end_chars.is_empty() {
                        state = State::NotLooking;
                        reset_stop_start = true;
                        product = 1;
                        continue;
                    }
                } else {
                    reset_stop_start = true;
                }

                if *next_char == ' ' {
                    if chr.is_ascii_digit() {
                        state = State::CollectingDigits;
                        let _ = mul_chars.pop();
                        digits = Vec::new();
                        let _ = digits.push(chr);
                    } else {
                        reset = true;
                    }
                    continue;
                }

                if *next_char == chr {
                    match mul_chars.pop() {
                        Some(_) => continue,
                        None => defmt::unreachable!(),
                        // will be reached in CollectingDigits state
                    }
                } else {
                    reset = true;
                }
            }
            State::CollectingDigits => {
                if chr.is_ascii_digit() {
                    match digits.push(chr) {
                        Ok(_) => {}
                        Err(_) => {
                            reset = true;
                        }
                    }
                } else if chr == *next_char {
                    let all_digits: String<MAX_DIGITS> = digits.iter().collect();
                    product *= all_digits.parse::<i32>().unwrap();
                    state = State::LookingForMul;
                    let _ = mul_chars.pop();

                    if mul_chars.is_empty() {
                        total += product;
                        reset = true;
                    }
                    continue;
                } else {
                    state = State::LookingForMul;
                    reset = true;
                }
            }
            State::NotLooking => {
                if *next_start_char == chr {
                    let _ = start_chars.pop();

                    if start_chars.is_empty() {
                        state = State::LookingForMul;
                        reset_stop_start = true;
                        continue;
                    }
                } else {
                    reset_stop_start = true;
                }
            }
        }
    }

    total
}

#[entry]
fn main() -> ! {
    info!("Program start");

    let inputs = include_str!("../input.txt");

    let ans1 = part1(inputs);
    let ans2 = part2(inputs);
    info!("calculation finished");

    // forcing the compiler to keeps these alive so I can view them
    // with the debugger
    black_box(ans1);
    black_box(ans2);
    loop {}
}
