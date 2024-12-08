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

const MAX_NODES: usize = 1000;

fn gcd(mut a: i8, mut b: i8) -> i8 {
    // with thanks to ChatGPT / Euclid
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn part1(nodes: &Vec<(i8, i8, u8), MAX_NODES>, maxrow: i8, maxcol: i8) -> usize {
    let mut antinodes: Vec<(i8, i8), MAX_NODES> = Vec::new();

    for (later_idx, later_node) in nodes.iter().enumerate() {
        for (earlier_idx, earlier_node) in nodes.iter().enumerate() {
            if (earlier_idx >= later_idx) || later_node.2 != earlier_node.2 {
                continue;
            }

            let delta_row = later_node.0 - earlier_node.0;
            let delta_col = later_node.1 - earlier_node.1;

            let mut antinode = (earlier_node.0 - delta_row, earlier_node.1 - delta_col);
            if (0..=maxrow).contains(&antinode.0)
                && (0..=maxcol).contains(&antinode.1)
                && !antinodes.contains(&antinode)
            {
                let _ = antinodes.push(antinode);
            }

            antinode = (later_node.0 + delta_row, later_node.1 + delta_col);
            if (0..=maxrow).contains(&antinode.0)
                && (0..=maxcol).contains(&antinode.1)
                && !antinodes.contains(&antinode)
            {
                let _ = antinodes.push(antinode);
            }
        }
    }

    antinodes.len()
}

fn part2(nodes: &Vec<(i8, i8, u8), MAX_NODES>, maxrow: i8, maxcol: i8) -> usize {
    let mut antinodes: Vec<(i8, i8), MAX_NODES> = Vec::new();

    for (later_idx, later_node) in nodes.iter().enumerate() {
        for (earlier_idx, earlier_node) in nodes.iter().enumerate() {
            if (earlier_idx >= later_idx) || later_node.2 != earlier_node.2 {
                continue;
            }

            let mut delta_row = later_node.0 - earlier_node.0;
            let mut delta_col = later_node.1 - earlier_node.1;
            let delta_gcd = gcd(delta_row, delta_col);
            delta_row /= delta_gcd;
            delta_col /= delta_gcd;

            for multiple in 0..i8::MAX {
                let antinode = (
                    earlier_node.0 - multiple * delta_row,
                    earlier_node.1 - multiple * delta_col,
                );
                if (0..=maxrow).contains(&antinode.0) && (0..=maxcol).contains(&antinode.1) {
                    if !antinodes.contains(&antinode) {
                        let _ = antinodes.push(antinode);
                    }
                } else {
                    break;
                }
            }

            for multiple in 0..i8::MAX {
                let antinode = (
                    later_node.0 + multiple * delta_row,
                    later_node.1 + multiple * delta_col,
                );
                if (0..=maxrow).contains(&antinode.0) && (0..=maxcol).contains(&antinode.1) {
                    if !antinodes.contains(&antinode) {
                        let _ = antinodes.push(antinode);
                    }
                } else {
                    break;
                }
            }
        }
    }

    antinodes.len()
}

fn count_antinodes(input: &str) -> (usize, usize) {
    let mut nodes: Vec<(i8, i8, u8), MAX_NODES> = Vec::new();
    let mut maxcol = 0;
    let mut maxrow = 0;

    for (row, line) in input.lines().enumerate() {
        for (col, chr) in line.chars().enumerate() {
            if chr != '.' {
                let _ = nodes.push((row as i8, col as i8, chr as u8));
            }
            if row == 0 {
                maxcol = col;
            }
        }
        maxrow = row;
    }

    (
        part1(&nodes, maxrow as i8, maxcol as i8),
        part2(&nodes, maxrow as i8, maxcol as i8),
    )
}

#[entry]
fn main() -> ! {
    info!("Program start");

    let inputs = include_str!("../input.txt");

    let ans1 = count_antinodes(inputs);
    info!("calculation finished");

    // forcing the compiler to keeps these alive so I can view them
    // with the debugger
    black_box(ans1);
    loop {}
}
