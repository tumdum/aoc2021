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

fn add(m: &mut [Option<u64>; 9], idx: usize, dif: u64) {
    let entry = &mut m[idx];
    match entry {
        Some(v) => *entry = Some(*v + dif),
        None => *entry = Some(dif),
    }
}

fn grow_fishes(fishes: &[u8]) -> (u64, u64) {
    let mut part1 = 0;
    let mut counts: [Option<u64>; 9] = [None; 9];
    fishes.iter().for_each(|f| add(&mut counts, *f as usize, 1));
    for day in 0..256 {
        if day == 80 {
            part1 = counts.iter().flatten().sum();
        }
        let mut new_counts = [None; 9];
        for days in 0..counts.len() {
            if let Some(count) = counts[days] {
                if days == 0 {
                    add(&mut new_counts, 6, count);
                } else {
                    add(&mut new_counts, days - 1, count);
                }
            }
        }
        add(&mut new_counts, 8, counts[0].unwrap_or_default());
        counts = new_counts;
    }
    (part1, counts.iter().flatten().sum())
}
