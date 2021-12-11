use std::io::BufRead;
use std::time::{Duration, Instant};

type V = smallvec::SmallVec<[u8; 16]>;

fn check_corrupted(s: &[u8]) -> Result<V, u8> {
    let mut stack = V::new();
    for c in s {
        debug_assert!(!stack.spilled());
        if *c < 4 {
            stack.push(*c);
        } else if inv(stack.pop().unwrap()) != *c {
            return Err(*c);
        }
    }
    debug_assert!(!stack.spilled());
    Ok(stack)
}

const fn map(c: char) -> u8 {
    match c {
        '(' => 0,
        '[' => 1,
        '{' => 2,
        '<' => 3,
        ')' => 4,
        ']' => 5,
        '}' => 6,
        '>' => 7,
        _ => u8::max_value(),
    }
}

const fn inv(c: u8) -> u8 {
    c + 4
}

const fn cost(c: u8) -> usize {
    match c {
        4 => 3,
        5 => 57,
        6 => 1197,
        7 => 25137,
        _ => usize::max_value(),
    }
}

const fn score(c: u8) -> usize {
    c as usize - 3
}

pub fn solve(input: &mut dyn BufRead, verify_expected: bool, output: bool) -> Duration {
    let input: Vec<V> = input
        .lines()
        .map(|s| s.unwrap().chars().map(map).collect())
        .collect();

    let s = Instant::now();

    let mut part1 = 0;
    let mut scores = vec![];

    for e in input.iter().map(|l| check_corrupted(l)) {
        match e {
            Ok(s) => scores.push(
                s.into_iter()
                    .map(inv)
                    .rev()
                    .fold(0, |tot, c| tot * 5 + score(c)),
            ),
            Err(c) => {
                part1 += cost(c);
            }
        }
    }

    let mid = scores.len() / 2;
    let (_, part2, _) = scores.select_nth_unstable(mid);
    let part2 = *part2;

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(215229, part1);
        assert_eq!(1105996483, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    e
}
