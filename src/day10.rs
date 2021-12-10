use std::collections::VecDeque;
use std::io::BufRead;
use std::time::{Duration, Instant};

fn check_corrupted(s: &[char]) -> Result<VecDeque<char>, char> {
    let mut stack = VecDeque::new();
    for c in s {
        if *c == '(' || *c == '[' || *c == '{' || *c == '<' {
            stack.push_back(*c);
        } else if inv(stack.pop_back().unwrap()) != *c {
            return Err(*c);
        }
    }
    Ok(stack)
}

fn inv(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

fn cost(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn score(c: char) -> usize {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unreachable!(),
    }
}

pub fn solve(input: &mut dyn BufRead, verify_expected: bool, output: bool) -> Duration {
    let input: Vec<Vec<char>> = input
        .lines()
        .map(|s| s.unwrap().chars().collect())
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
