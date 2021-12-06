use std::io::BufRead;
use std::time::{Duration, Instant};

pub fn solve(input: &mut dyn BufRead, verify_expected: bool, output: bool) -> Duration {
    let input: Vec<u8> = input
        .lines()
        .map(|s| s.unwrap())
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let s = Instant::now();
    let (part1, part2) = grow_fishes(&input);
    let e = s.elapsed();
    if verify_expected {
        assert_eq!(362666, part1);
        assert_eq!(1640526601595, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    e
}

fn add(m: &mut [Option<usize>; 9], idx: usize, dif: usize) {
    let entry = &mut m[idx];
    match entry {
        Some(v) => *entry = Some(*v + dif),
        None => *entry = Some(dif),
    }
}

fn grow_fishes(fishes: &[u8]) -> (usize, usize) {
    let mut part1 = 0;
    let mut age_to_count: [Option<usize>; 9] = [None; 9];
    for f in fishes {
        add(&mut age_to_count, *f as usize, 1);
    }
    for day in 0..256 {
        if day == 80 {
            part1 = age_to_count.iter().flat_map(|v| v).sum();
        }
        let mut new = 0;
        let mut new_age_to_count = [None; 9];
        for days in 0..age_to_count.len() {
            if let Some(count) = age_to_count[days] {
                if days == 0 {
                    add(&mut new_age_to_count, 6, count);
                    new += count;
                } else {
                    add(&mut new_age_to_count, days - 1, count);
                }
            }
        }
        add(&mut new_age_to_count, 8, new);
        age_to_count = new_age_to_count;
    }
    (part1, age_to_count.iter().flat_map(|v| v).sum())
}
