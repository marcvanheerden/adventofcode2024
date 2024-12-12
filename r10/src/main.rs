#![no_std]
#![no_main]

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use rp_pico as bsp;

use core::hint::black_box;
use core::str;
use heapless::{Deque, Vec};

const MAX_PTS: usize = 2200;
const MAX_TASKS: usize = 2000;
const MAX_DEST: usize = 500;

#[derive(Debug)]
struct Map {
    pts: Vec<u8, MAX_PTS>,
    height: usize,
    width: usize,
}

impl Map {
    fn new(input: &str) -> Self {
        let width = input.find('\n').unwrap();
        let height = input.len() / (width + 1);
        let pts = input
            .chars()
            .filter(|&c| c != '\n')
            .map(|c| c as u8 - 48)
            .collect();

        Self { pts, height, width }
    }

    fn get(&self, row: usize, col: usize) -> Option<u8> {
        if (row < self.height) && (col < self.width) {
            return Some(self.pts[row * self.width + col]);
        }

        None
    }

    fn next_steps(&self, row: usize, col: usize) -> Vec<(usize, usize), 4> {
        let value = self.get(row, col).unwrap();
        let mut next = Vec::new();

        if let Some(val) = self.get(row.wrapping_sub(1), col) {
            if (value + 1) == val {
                next.push((row - 1, col));
            }
        }

        if let Some(val) = self.get(row, col.wrapping_sub(1)) {
            if (value + 1) == val {
                next.push((row, col - 1));
            }
        }

        if let Some(val) = self.get(row + 1, col) {
            if (value + 1) == val {
                next.push((row + 1, col));
            }
        }

        if let Some(val) = self.get(row, col + 1) {
            if (value + 1) == val {
                next.push((row, col + 1));
            }
        }

        next
    }

    fn trail_head_value1(&self, row: usize, col: usize) -> usize {
        let mut tasks: Deque<(usize, usize), MAX_TASKS> = Deque::new();
        tasks.push_front((row, col));

        let mut destinations: Vec<(usize, usize), MAX_DEST> = Vec::new();
        while !tasks.is_empty() {
            let task = tasks.pop_front().unwrap();

            if let Some(val) = self.get(task.0, task.1) {
                if val == 9 {
                    if !destinations.contains(&task) {
                        destinations.push(task);
                    }
                } else {
                    for new_task in self.next_steps(task.0, task.1).into_iter() {
                        tasks.push_back(new_task);
                    }
                }
            }
        }

        destinations.len()
    }

    fn trail_head_value2(&self, row: usize, col: usize) -> usize {
        let mut tasks: Deque<(usize, usize), MAX_TASKS> = Deque::new();
        tasks.push_front((row, col));

        let mut total = 0;
        while !tasks.is_empty() {
            let task = tasks.pop_front().unwrap();

            if let Some(val) = self.get(task.0, task.1) {
                if val == 9 {
                    total += 1;
                } else {
                    for new_task in self.next_steps(task.0, task.1).into_iter() {
                        tasks.push_back(new_task);
                    }
                }
            }
        }

        total
    }

    fn solution(&self) -> (usize, usize) {
        let mut total1 = 0;
        let mut total2 = 0;

        for row in 0..self.height {
            for col in 0..self.width {
                if let Some(val) = self.get(row, col) {
                    if val == 0 {
                        total1 += self.trail_head_value1(row, col);
                        total2 += self.trail_head_value2(row, col);
                    }
                }
            }
        }

        (total1, total2)
    }
}
#[entry]
fn main() -> ! {
    info!("Program start");

    let inputs = include_str!("../input.txt");
    let map = Map::new(inputs);
    let output = map.solution();

    info!("calculation finished");

    // forcing the compiler to keeps these alive so I can view them
    // with the debugger
    black_box(output);
    loop {}
}
