use itertools::{iproduct, iterate};
use rustc_hash::FxHashSet;
use std::collections::VecDeque;
use std::io::BufRead;
use std::time::{Duration, Instant};

const P1_STEPS: usize = 100;
type V<T> = smallvec::SmallVec<[T; 10]>;

const OFFSETS: [(i8, i8); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn update(
    v: &mut [V<i8>],
    (r, c): (i8, i8),
    todo: &mut VecDeque<(i8, i8)>,
    all_flashes: &mut FxHashSet<(i8, i8)>,
) {
    v[r as usize][c as usize] += 1;
    if v[r as usize][c as usize] > 9 {
        v[r as usize][c as usize] = 0;
        todo.push_back((r as i8, c as i8));
        all_flashes.insert((r as i8, c as i8));
    }
}

fn step(v: &mut V<V<i8>>) -> usize {
    let mut todo = VecDeque::default();
    let mut all_flashes = FxHashSet::default();
    let l = v.len() as i8;

    iproduct!(0..l, 0..l).for_each(|p| update(v, p, &mut todo, &mut all_flashes));

    while let Some((r, c)) = todo.pop_front() {
        v[r as usize][c as usize] = 0;
        for (r, c) in OFFSETS.iter().map(|(rd, cd)| (r + rd, c + cd)) {
            if r >= 0 && c >= 0 && r < l && c < l && !all_flashes.contains(&(r, c)) {
                update(v, (r, c), &mut todo, &mut all_flashes);
            }
        }
    }
    all_flashes.len()
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
