use std::io::BufRead;
use std::time::{Duration, Instant};

fn cost(f: i32, t: i32) -> i32 {
    let n = (f - t).abs();
    n * (n + 1) / 2
}

fn total(from: i32, all: &Vec<i32>) -> i32 {
    all.iter().map(|d| (from - d).abs()).sum()
}

fn total_part2(from: i32, all: &Vec<i32>) -> i32 {
    all.iter().map(|d| cost(*d, from)).sum()
}

pub fn solve(input: &mut dyn BufRead, verify_expected: bool, output: bool) -> Duration {
    let input: Vec<i32> = input
        .lines()
        .map(|s| s.unwrap())
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let s = Instant::now();

    // NOTE: can be easily speedup with par_iter

    let part1 = input.iter().map(|s| total(*s, &input)).min().unwrap();

    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    let part2 = (min..=max).map(|s| total_part2(s, &input)).min().unwrap();

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
