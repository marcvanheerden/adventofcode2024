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

const MAX_ROWS: usize = 140;
const MAX_COLS: usize = 140;
const MAX_WORD_LEN: usize = 4;
type Map = Vec<Vec<char, MAX_COLS>, MAX_ROWS>;

fn part1(input: &str, word: &str) -> u32 {
    let mut map: Map = Vec::new();
    let mut count = 0;
    let rev_word: String<MAX_WORD_LEN> = word.chars().rev().collect();

    for line in input.lines() {
        let _ = map.push(line.chars().collect());
    }

    for row in 0..map.len() {
        for col in 0..map.first().unwrap().len() {
            let row_margin = row <= (map.len() - word.len());
            let col_margin = col <= (map.first().unwrap().len() - word.len());

            if col_margin {
                if find_horizontal(&map, word, row, col)
                    || find_horizontal(&map, &rev_word, row, col)
                {
                    count += 1;
                }
            }

            if row_margin {
                if find_vertical(&map, word, row, col) ||
                    find_vertical(&map, &rev_word, row, col) {
                    count += 1;
                }
            }

            if row_margin && col_margin {
                if find_pos_diag(&map, word, row, col) ||
                    find_pos_diag(&map, &rev_word, row, col) {
                    count += 1;
                }
                if find_neg_diag(&map, word, row, col) ||
                    find_neg_diag(&map, &rev_word, row, col) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn part2(input: &str, word: &str) -> u32 {
    let mut map: Map = Vec::new();
    let mut count = 0;
    let rev_word: String<MAX_WORD_LEN> = word.chars().rev().collect();

    for line in input.lines() {
        let _ = map.push(line.chars().collect());
    }

    for row in 0..(map.len() - word.len() + 1) {
        for col in 0..(map.first().unwrap().len() - word.len() + 1) {
            let pos = find_pos_diag(&map, word, row, col);
            let pos_rev = find_pos_diag(&map, &rev_word, row, col);
            let neg = find_neg_diag(&map, word, row, col);
            let neg_rev = find_neg_diag(&map, &rev_word, row, col);

            if (pos || pos_rev) && (neg || neg_rev) {
                count += 1;
            }
        }
    }

    count
}

fn find_horizontal(map: &Map, word: &str, row: usize, col: usize) -> bool {
    word.chars()
        .zip(map[row][col..].iter())
        .all(|(a, b)| a == *b)
}

fn find_vertical(map: &Map, word: &str, row: usize, col: usize) -> bool {
    map.iter()
        .skip(row)
        .take(word.len())
        .filter_map(|row_| row_.get(col))
        .zip(word.chars())
        .all(|(a, b)| *a == b)
}

fn find_pos_diag(map: &Map, word: &str, row: usize, col: usize) -> bool {
    map.iter()
        .skip(row)
        .take(word.len())
        .enumerate()
        .filter_map(|(idx, row_)| row_.get(col + word.len() - 1 - idx))
        .zip(word.chars())
        .all(|(a, b)| *a == b)
}

fn find_neg_diag(map: &Map, word: &str, row: usize, col: usize) -> bool {
    map.iter()
        .skip(row)
        .take(word.len())
        .enumerate()
        .filter_map(|(idx, row_)| row_.get(col + idx))
        .zip(word.chars())
        .all(|(a, b)| *a == b)
}

#[entry]
fn main() -> ! {
    info!("Program start");

    let inputs = include_str!("../input.txt");

    let ans1 = part1(inputs, "XMAS");
    let ans2 = part2(inputs, "MAS");
    info!("calculation finished");

    // forcing the compiler to keeps these alive so I can view them
    // with the debugger
    black_box(ans1);
    black_box(ans2);
    loop {}
}
