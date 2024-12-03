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

struct StateMachine1 {
    state: State,
    total: i32,
    product: i32,
    digits: Vec<char, MAX_DIGITS>,
    mul_chars: Vec<char, MUL_PATTERN_LEN>,
}

impl StateMachine1 {
    fn new() -> Self {
        Self {
            state: State::LookingForMul,
            total: 0,
            product: 1,
            digits: Vec::new(),
            mul_chars: Vec::from_slice(MUL_PATTERN).unwrap(),
        }
    }

    fn reset(&mut self) {
        if self.mul_chars.len() < MUL_PATTERN_LEN {
            self.mul_chars = Vec::from_slice(MUL_PATTERN).unwrap();
        }
        self.digits = Vec::new();
        self.product = 1;
    }
}

fn part1(input: &str) -> i32 {
    let mut sm = StateMachine1::new();

    for chr in input.chars() {
        let next_char = sm.mul_chars.last().unwrap();
        match sm.state {
            State::LookingForMul => {
                if *next_char == ' ' {
                    if chr.is_ascii_digit() {
                        sm.state = State::CollectingDigits;
                        let _ = sm.mul_chars.pop();
                        sm.digits = Vec::new();
                        let _ = sm.digits.push(chr);
                    } else {
                        sm.reset();
                    }
                    continue;
                }

                if *next_char == chr {
                    match sm.mul_chars.pop() {
                        Some(_) => continue,
                        None => defmt::unreachable!(),
                        // will be reached in CollectingDigits state
                    }
                } else {
                    sm.reset();
                }
            }
            State::CollectingDigits => {
                if chr.is_ascii_digit() {
                    match sm.digits.push(chr) {
                        Ok(_) => {}
                        Err(_) => {
                            sm.reset();
                        }
                    }
                } else if chr == *next_char {
                    let all_digits: String<MAX_DIGITS> = sm.digits.iter().collect();
                    sm.product *= all_digits.parse::<i32>().unwrap();
                    sm.state = State::LookingForMul;
                    let _ = sm.mul_chars.pop();

                    if sm.mul_chars.is_empty() {
                        sm.total += sm.product;
                        sm.reset();
                    }
                    continue;
                } else {
                    sm.state = State::LookingForMul;
                    sm.reset();
                }
            }
            State::NotLooking => continue,
        }
    }

    sm.total
}

struct StateMachine2 {
    state: State,
    total: i32,
    product: i32,
    digits: Vec<char, MAX_DIGITS>,
    mul_chars: Vec<char, MUL_PATTERN_LEN>,
    start_chars: Vec<char, START_PATTERN_LEN>,
    end_chars: Vec<char, END_PATTERN_LEN>,
}

impl StateMachine2 {
    fn new() -> Self {
        Self {
            state: State::LookingForMul,
            total: 0,
            product: 1,
            digits: Vec::new(),
            mul_chars: Vec::from_slice(MUL_PATTERN).unwrap(),
            start_chars: Vec::from_slice(START_PATTERN).unwrap(),
            end_chars: Vec::from_slice(END_PATTERN).unwrap(),
        }
    }

    fn reset(&mut self) {
        if self.mul_chars.len() < MUL_PATTERN_LEN {
            self.mul_chars = Vec::from_slice(MUL_PATTERN).unwrap();
        }
        self.digits = Vec::new();
        self.product = 1;
    }

    fn reset_stop_start(&mut self) {
        if self.start_chars.len() < START_PATTERN_LEN {
            self.start_chars = Vec::from_slice(START_PATTERN).unwrap();
        }
        if self.end_chars.len() < END_PATTERN_LEN {
            self.end_chars = Vec::from_slice(END_PATTERN).unwrap();
        }
    }
}
fn part2(input: &str) -> i32 {
    let mut sm = StateMachine2::new();

    for chr in input.chars() {
        let next_char = *sm.mul_chars.last().unwrap();
        let next_start_char = sm.start_chars.last().unwrap();
        let next_end_char = sm.end_chars.last().unwrap();

        match sm.state {
            State::LookingForMul => {
                if *next_end_char == chr {
                    let _ = sm.end_chars.pop();

                    if sm.end_chars.is_empty() {
                        sm.state = State::NotLooking;
                        sm.reset_stop_start();
                        sm.product = 1;
                        continue;
                    }
                } else {
                    sm.reset_stop_start();
                }

                if next_char == ' ' {
                    if chr.is_ascii_digit() {
                        sm.state = State::CollectingDigits;
                        let _ = sm.mul_chars.pop();
                        sm.digits = Vec::new();
                        let _ = sm.digits.push(chr);
                    } else {
                        sm.reset();
                    }
                    continue;
                }

                if next_char == chr {
                    match sm.mul_chars.pop() {
                        Some(_) => continue,
                        None => defmt::unreachable!(),
                        // will be reached in CollectingDigits state
                    }
                } else {
                    sm.reset();
                }
            }
            State::CollectingDigits => {
                if chr.is_ascii_digit() {
                    match sm.digits.push(chr) {
                        Ok(_) => {}
                        Err(_) => {
                            sm.reset();
                        }
                    }
                } else if chr == next_char {
                    let all_digits: String<MAX_DIGITS> = sm.digits.iter().collect();
                    sm.product *= all_digits.parse::<i32>().unwrap();
                    sm.state = State::LookingForMul;
                    let _ = sm.mul_chars.pop();

                    if sm.mul_chars.is_empty() {
                        sm.total += sm.product;
                        sm.reset();
                    }
                    continue;
                } else {
                    sm.state = State::LookingForMul;
                    sm.reset();
                }
            }
            State::NotLooking => {
                if *next_start_char == chr {
                    let _ = sm.start_chars.pop();

                    if sm.start_chars.is_empty() {
                        sm.state = State::LookingForMul;
                        sm.reset_stop_start();
                        continue;
                    }
                } else {
                    sm.reset_stop_start();
                }
            }
        }
    }

    sm.total
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
