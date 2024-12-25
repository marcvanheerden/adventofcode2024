const PINCOUNT: usize = 5;
const ROWMAX: u8 = 6;

#[derive(Debug)]
enum LockNKey {
    Lock([u8; PINCOUNT]),
    Key([u8; PINCOUNT]),
}

fn parse_lock_or_key(input: &str) -> LockNKey {
    let mut pins = [u8::MAX; PINCOUNT];
    if input.lines().next().unwrap() == "#####" {
        for (line_no, line) in input.lines().enumerate() {
            for (col, chr) in line.chars().enumerate() {
                if chr != '#' && pins[col] == u8::MAX {
                    pins[col] = line_no as u8 - 1;
                }
            }
        }
        assert!(pins.iter().max().unwrap() < &u8::MAX);
        return LockNKey::Lock(pins);
    }
    assert_eq!(input.lines().next().unwrap(), ".....");

    for (line_no, line) in input.lines().rev().enumerate() {
        for (col, chr) in line.chars().enumerate() {
            if chr != '#' && pins[col] == u8::MAX {
                pins[col] = line_no as u8 - 1;
            }
        }
    }
    assert!(pins.iter().max().unwrap() < &u8::MAX);

    LockNKey::Key(pins)
}

fn main() {
    let input = include_str!("../input.txt");

    let locksnkeys: Vec<_> = input.split("\n\n").map(parse_lock_or_key).collect();
    let mut matches = 0;

    for lk1 in locksnkeys.iter() {
        for lk2 in locksnkeys.iter() {
            if let (LockNKey::Lock(lockpins), LockNKey::Key(keypins)) = (lk1, lk2) {
                if lockpins
                    .iter()
                    .zip(keypins)
                    .all(|(p1, p2)| p1 + p2 < ROWMAX)
                {
                    matches += 1;
                }
            }
        }
    }

    dbg!(matches);
}
