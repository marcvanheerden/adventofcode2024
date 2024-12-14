use std::collections::HashSet;

fn wrap(a: i16, b: i16, max: i16) -> i16 {
    ((a + b) % max + max) % max
}

const DIMENSIONS: (i16, i16) = (101, 103);

#[derive(Debug, PartialEq, Clone)]
struct Bot {
    pos: (i16, i16),
    vel: (i16, i16),
}

impl Bot {
    fn new(input: &str) -> Option<Self> {
        let split1 = input.replace("p=", "");
        let mut split1 = split1.split(" v=");
        let mut pos_split = split1.next()?.split(',');
        let mut vel_split = split1.next()?.split(',');

        Some(Self {
            pos: (
                pos_split.next()?.parse().expect("not a number"),
                pos_split.next()?.parse().expect("not a number"),
            ),
            vel: (
                vel_split.next()?.parse().expect("not a number"),
                vel_split.next()?.parse().expect("not a number"),
            ),
        })
    }

    fn step(&mut self) {
        self.pos.0 = wrap(self.pos.0, self.vel.0, DIMENSIONS.0);
        self.pos.1 = wrap(self.pos.1, self.vel.1, DIMENSIONS.1);
    }

    fn quadrant(&self) -> Option<usize> {
        if self.pos.0 < (DIMENSIONS.0 / 2) {
            if self.pos.1 < (DIMENSIONS.1 / 2) {
                return Some(0);
            }

            if self.pos.1 > (DIMENSIONS.1 / 2) {
                return Some(1);
            }
        }

        if self.pos.0 > (DIMENSIONS.0 / 2) {
            if self.pos.1 < (DIMENSIONS.1 / 2) {
                return Some(2);
            }

            if self.pos.1 > (DIMENSIONS.1 / 2) {
                return Some(3);
            }
        }

        None
    }
}

fn display(bots: &[Bot]) {
    let pos: HashSet<_> = bots.iter().map(|b| b.pos).collect();

    for x in 0..DIMENSIONS.0 {
        for y in 0..DIMENSIONS.1 {
            if pos.contains(&(x, y)) {
                print!("##");
            } else {
                print!("  ");
            }
        }
        println!();
    }
}

fn closeness(bots: &[Bot]) -> usize {
    let pos: HashSet<_> = bots.iter().map(|b| b.pos).collect();

    let mut touching = 0;
    for bot1 in pos.iter() {
        for bot2 in pos.iter() {
            if bot1 != bot2 && (bot1.0 - bot2.0).abs() <= 1 && (bot1.1 - bot2.1).abs() <= 1 {
                touching += 1;
                break;
            }
        }
    }

    touching * 100 / pos.len()
}

fn main() {
    let input = include_str!("../input.txt");

    let mut bots: Vec<_> = input.lines().filter_map(Bot::new).collect();
    let mut bots1 = bots.clone();
    for _ in 1..=100 {
        bots1.iter_mut().for_each(|b| b.step());
    }

    let mut quads = [0u32; 4];
    for bot in bots1.iter() {
        if let Some(quad) = bot.quadrant() {
            quads[quad] += 1;
        }
    }

    dbg!(quads[0] * quads[1] * quads[2] * quads[3]);

    for step in 1..10000000 {
        bots.iter_mut().for_each(|b| b.step());

        if closeness(&bots) > 60 {
            println!("---------------------------------------------------------------------------------------- {}", step);
            display(&bots);
            break;
        }
    }
}
