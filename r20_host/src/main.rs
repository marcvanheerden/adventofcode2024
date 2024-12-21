fn parse_track(input: &str) -> Vec<(u8, u8)> {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut track = Vec::new();

    for (row, line) in input.lines().enumerate() {
        for (col, chr) in line.chars().enumerate() {
            match chr {
                '.' => {
                    track.push((row as u8, col as u8));
                }
                'S' => {
                    start = (row as u8, col as u8);
                }
                'E' => {
                    track.push((row as u8, col as u8));
                    end = (row as u8, col as u8);
                }
                _ => {}
            }
        }
    }

    let mut ordered_track = vec![start];

    while *ordered_track.last().unwrap() != end {
        let mut to_remove = 0;
        for (idx, pos) in track.iter().enumerate() {
            if manhattan_dist(pos, ordered_track.last().unwrap()) == 1 {
                ordered_track.push(*pos);
                to_remove = idx;
                break;
            }
        }
        track.remove(to_remove);
    }

    ordered_track
}

fn count_short_cuts(track: &[(u8, u8)], max_cheat: u16, faster: usize) -> u32 {
    let mut total = 0;
    for (earlier_idx, earlier_loc) in track.iter().enumerate() {
        for (later_idx, later_loc) in track.iter().enumerate().skip(earlier_idx + 1) {
            let dist = manhattan_dist(earlier_loc, later_loc);
            if (2..=max_cheat).contains(&dist)
                && (later_idx - earlier_idx - dist as usize) >= faster
            {
                total += 1;
            }
        }
    }

    total
}

fn manhattan_dist(point_a: &(u8, u8), point_b: &(u8, u8)) -> u16 {
    point_a.0.abs_diff(point_b.0) as u16 + point_a.1.abs_diff(point_b.1) as u16
}

fn main() {
    let input = include_str!("../input.txt");
    let track = parse_track(input);
    dbg!(count_short_cuts(&track, 2, 100));
    dbg!(count_short_cuts(&track, 20, 100));
}
