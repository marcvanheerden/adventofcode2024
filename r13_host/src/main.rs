use std::cmp::Ordering;

const ACOST: u64 = 3;
const BCOST: u64 = 1;
const LEARNING_RATIO: f64 = 100.0;
const SEARCHES_PER_ROUND: usize = 100_000;

#[derive(Debug)]
struct ClawMachine {
    a_move: (u64, u64),
    b_move: (u64, u64),
    prize: (u64, u64),
}

impl ClawMachine {
    fn new(input: &str) -> Self {
        let mut line_iter = input.lines();

        let mut split1 = line_iter.next().unwrap().split('+').skip(1);
        let ax = split1.next().unwrap().split(',').next().unwrap();
        let ay = split1.next().unwrap().trim();

        let mut split2 = line_iter.next().unwrap().split('+').skip(1);
        let bx = split2.next().unwrap().split(',').next().unwrap();
        let by = split2.next().unwrap().trim();

        let mut split3 = line_iter.next().unwrap().split('=').skip(1);
        let prize_x = split3.next().unwrap().split(',').next().unwrap();
        let prize_y = split3.next().unwrap().trim();

        Self {
            a_move: (ax.parse().unwrap(), ay.parse().unwrap()),
            b_move: (bx.parse().unwrap(), by.parse().unwrap()),
            prize: (prize_x.parse().unwrap(), prize_y.parse().unwrap()),
        }
    }

    fn test(&self, candidate: (u64, u64)) -> Ordering {
        let x = candidate.0 * self.a_move.0 + candidate.1 * self.b_move.0;
        let y = candidate.0 * self.a_move.1 + candidate.1 * self.b_move.1;

        if x < self.prize.0 || y < self.prize.1 {
            Ordering::Less
        } else if x == self.prize.0 && y == self.prize.1 {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }

    fn optimise(&self) -> Option<u64> {
        let mut a_pushes = 0;
        let mut b_pushes =
            std::cmp::max(self.prize.0 / self.b_move.0, self.prize.1 / self.b_move.1) - 1;

        while self.test((a_pushes, b_pushes)) == Ordering::Less {
            b_pushes += 1;
        }

        loop {
            let start_a_pushes = a_pushes;
            let start_b_pushes = b_pushes;
            let start_miss = self
                .prize
                .0
                .abs_diff(a_pushes * self.a_move.0 + b_pushes * self.b_move.0)
                + self
                    .prize
                    .1
                    .abs_diff(a_pushes * self.a_move.1 + b_pushes * self.b_move.1);

            for _ in 0..SEARCHES_PER_ROUND {
                let limit = b_pushes == 0;
                match self.test((a_pushes, b_pushes)) {
                    Ordering::Less => a_pushes += 1,
                    Ordering::Equal => {
                        return Some(ACOST * a_pushes + BCOST * b_pushes);
                    }
                    Ordering::Greater => b_pushes -= 1,
                }
                if limit {
                    return None;
                }
            }

            let end_miss = self
                .prize
                .0
                .abs_diff(a_pushes * self.a_move.0 + b_pushes * self.b_move.0)
                + self
                    .prize
                    .1
                    .abs_diff(a_pushes * self.a_move.1 + b_pushes * self.b_move.1);

            if end_miss >= start_miss {
                return None;
            }

            let progress = (start_miss as f64 - end_miss as f64) / (start_miss as f64);
            let delta_a = (a_pushes as f64 - start_a_pushes as f64) / progress;
            a_pushes = (start_a_pushes as f64 + delta_a / LEARNING_RATIO) as u64;
            let delta_b = (b_pushes as f64 - start_b_pushes as f64) / progress;
            b_pushes = (start_b_pushes as f64 + delta_b / LEARNING_RATIO) as u64;
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let mut machines: Vec<_> = input.split("\n\n").map(ClawMachine::new).collect();
    let ans1 = machines.iter().filter_map(|m| m.optimise()).sum::<u64>();
    dbg!(ans1);

    let increase = 10_000_000_000_000u64;
    for machine in machines.iter_mut() {
        machine.prize = (machine.prize.0 + increase, machine.prize.1 + increase);
    }
    let ans2 = machines.iter().filter_map(|m| m.optimise()).sum::<u64>();
    dbg!(ans2);
}
