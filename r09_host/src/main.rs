use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct Segment {
    id: Option<u64>,
    len: u8,
}

fn checksum(segs: &[Segment]) -> u64 {
    let mut checksum_idx = 0;
    let mut checksum = 0;

    for seg in segs.iter() {
        for _ in 0..seg.len {
            if let Some(id) = seg.id {
                checksum += id * checksum_idx;
            }
            checksum_idx += 1;
        }
    }

    checksum
}

fn process_inputs(input: &str) -> Vec<Segment> {
    let mut file = true;
    let mut segs = Vec::new();
    let mut id_no = 0;

    for chr in input.lines().next().unwrap().chars() {
        let id = if file { Some(id_no) } else { None };
        let len = chr as u8 - 48;
        if len > 0 {
            segs.push(Segment { id, len });
        }
        if file {
            id_no += 1;
        }

        file = !file;
    }

    segs
}

fn part1(mut segs: Vec<Segment>) -> u64 {
    let mut front_idx = 0usize;
    let mut back_idx = segs.len() - 1;

    while front_idx < back_idx {
        if segs[front_idx].id.is_some() {
            front_idx += 1;
            continue;
        }

        if segs[back_idx].id.is_none() {
            back_idx -= 1;
            continue;
        }

        match segs[front_idx].len.cmp(&segs[back_idx].len) {
            Ordering::Greater => {
                let copy = segs[back_idx].clone();
                segs[front_idx].len -= copy.len;
                segs[back_idx].id = None;
                segs.insert(front_idx, copy);
                back_idx += 1;
            }
            Ordering::Equal => {
                segs[front_idx].id = segs[back_idx].id;
                segs[back_idx].id = None;
            }
            Ordering::Less => {
                segs[back_idx].len -= segs[front_idx].len;
                segs[front_idx].id = segs[back_idx].id;
            }
        }
    }

    checksum(&segs)
}

fn part2(mut segs: Vec<Segment>) -> u64 {
    let mut back_idx = segs.len() - 1;

    while back_idx > 0 {
        if segs[back_idx].id.is_none() {
            back_idx -= 1;
            continue;
        }

        let mut front_idx = 0usize;
        while front_idx < back_idx {
            if segs[front_idx].id.is_some() {
                front_idx += 1;
                continue;
            }

            match segs[front_idx].len.cmp(&segs[back_idx].len) {
                Ordering::Greater => {
                    let copy = segs[back_idx].clone();
                    segs[front_idx].len -= copy.len;
                    segs[back_idx].id = None;
                    segs.insert(front_idx, copy);
                    back_idx += 1;
                    break;
                }
                Ordering::Equal => {
                    segs[front_idx].id = segs[back_idx].id;
                    segs[back_idx].id = None;
                    break;
                }
                Ordering::Less => {}
            }
            front_idx += 1;
        }
        back_idx -= 1;
    }

    checksum(&segs)
}

fn main() {
    let input = include_str!("../input.txt");
    let segments = process_inputs(input);

    dbg!(part1(segments.clone()));
    dbg!(part2(segments));
}
