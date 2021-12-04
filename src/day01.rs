use std::io::BufRead;
use std::time::{Duration, Instant};
pub fn solve(input: &mut dyn BufRead, verify_expected: bool) -> Duration {
    let input: Vec<i64> = input.lines().map(|s| s.unwrap().parse().unwrap()).collect();

    let s = Instant::now();
    let part1 = input.windows(2).filter(|w| w[0] < w[1]).count();
    let input: Vec<i64> = input.windows(3).map(|w| w.iter().sum()).collect();
    let part2 = input.windows(2).filter(|w| w[0] < w[1]).count();
    let e = s.elapsed();
    if verify_expected {
        assert_eq!(1602, part1);
        assert_eq!(1633, part2);
    }
    println!("\t{}", part1);
    println!("\t{}", part2);
    e
}
