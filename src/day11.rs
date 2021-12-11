use crate::pos::Pos;
use itertools::{iproduct, iterate};
use rustc_hash::FxHashSet;
use std::collections::VecDeque;
use std::io::BufRead;
use std::time::{Duration, Instant};

const P1_STEPS: usize = 100;
type V<T> = smallvec::SmallVec<[T; 10]>;

const OFFSETS: [Pos<i8>; 8] = [
    Pos(-1, -1),
    Pos(-1, 0),
    Pos(-1, 1),
    Pos(0, -1),
    Pos(0, 1),
    Pos(1, -1),
    Pos(1, 0),
    Pos(1, 1),
];

fn update(
    v: &mut [V<i8>],
    p: Pos<i8>,
    todo: &mut VecDeque<Pos<i8>>,
    flashes: &mut FxHashSet<Pos<i8>>,
) {
    if let Some(tmp) = p.get_mut(v) {
        *tmp += 1;
        if *tmp > 9 {
            *tmp = 0;
            todo.push_back(p);
            flashes.insert(p);
        }
    }
}

fn step(v: &mut V<V<i8>>) -> usize {
    let mut todo = VecDeque::default();
    let mut flashes = FxHashSet::default();
    let l = v.len() as i8;

    iproduct!(0..l, 0..l).for_each(|p| update(v, p.into(), &mut todo, &mut flashes));

    while let Some(next) = todo.pop_front() {
        *next.get_mut(v).unwrap() = 0;
        for neighbour in OFFSETS.iter().map(|delta| next + *delta) {
            if !flashes.contains(&neighbour) {
                update(v, neighbour, &mut todo, &mut flashes);
            }
        }
    }
    flashes.len()
}

pub fn solve(input: &mut dyn BufRead, verify_expected: bool, output: bool) -> Duration {
    let mut input: V<V<i8>> = input
        .lines()
        .map(|s| {
            s.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect()
        })
        .collect();

    let s = Instant::now();

    let part1 = (0..P1_STEPS).fold(0, |acc, _| acc + step(&mut input));
    let part2 = iterate(0, |_| step(&mut input))
        .take_while(|f| *f < 100)
        .count()
        + P1_STEPS;

    let e = s.elapsed();
    if verify_expected {
        assert_eq!(1637, part1);
        assert_eq!(242, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    e
}
