use std::io::BufRead;
use std::time::{Duration, Instant};

fn cost(f: i64, t: i64) -> i64 {
    let n = (f - t).abs();
    n * (n + 1) / 2
}

fn total(from: i64, all: &[i64]) -> i64 {
    all.iter().map(|d| (from - d).abs()).sum()
}

fn total_part2(from: i64, all: &[i64]) -> i64 {
    all.iter().map(|d| cost(*d, from)).sum()
}

pub fn solve(input: &mut dyn BufRead, verify_expected: bool, output: bool) -> Duration {
    let mut input: Vec<i64> = input
        .lines()
        .map(|s| s.unwrap())
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let s = Instant::now();

    input.sort_unstable();

    let part1 = total(input[input.len() / 2], &input);

    let avg: i64 = input.iter().sum::<i64>() / input.len() as i64;
    let part2 = ((avg - 1)..=(avg + 1))
        .map(|s| total_part2(s, &input))
        .min()
        .unwrap();

    let e = s.elapsed();
    if verify_expected {
        assert_eq!(333755, part1);
        assert_eq!(94017638, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    e
}
