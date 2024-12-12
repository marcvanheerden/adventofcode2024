use std::collections::HashMap;

fn stoney_gaze(num: u64, blinks_left: u8, memo: &mut HashMap<(u64, u8), u64>) -> u64 {
    if blinks_left == 0 {
        return 1;
    }

    if let Some(result) = memo.get(&(num, blinks_left)) {
        return *result;
    }

    let result = if num == 0 {
        stoney_gaze(1, blinks_left - 1, memo)
    } else if let Some((front, back)) = split_digits(num) {
        stoney_gaze(front, blinks_left - 1, memo) + stoney_gaze(back, blinks_left - 1, memo)
    } else {
        stoney_gaze(num * 2024, blinks_left - 1, memo)
    };

    let _ = memo.insert((num, blinks_left), result);

    result
}

fn split_digits(num: u64) -> Option<(u64, u64)> {
    let mut digits: u32 = 0;

    for idx in 1..30u32 {
        if num < 10u64.pow(idx) {
            digits = idx;
            break;
        }
    }

    if digits % 2 == 0 {
        let front = num / 10u64.pow(digits / 2);
        let back = num - (front * 10u64.pow(digits / 2));
        return Some((front, back));
    }

    None
}

fn blink_stones(input: &str, blinks: u8) -> u64 {
    let mut memo = HashMap::new();

    input
        .lines()
        .next()
        .unwrap()
        .split(' ')
        .map(|n| stoney_gaze(n.parse().unwrap(), blinks, &mut memo))
        .sum()
}

fn main() {
    let inputs = include_str!("../input.txt");
    let ans1 = blink_stones(inputs, 25);
    dbg!(ans1);
    let ans2 = blink_stones(inputs, 75);
    dbg!(ans2);
}
