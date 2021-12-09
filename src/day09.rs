use rustc_hash::FxHashSet;
use std::cmp::Reverse;
use std::io::BufRead;
use std::time::{Duration, Instant};

type I = i64;

fn get(input: &[Vec<I>], (x, y): (I, I)) -> Option<I> {
    input
        .get(x as usize)
        .and_then(|r| r.get(y as usize))
        .cloned()
}

fn around(input: &[Vec<I>], (x, y): (I, I)) -> impl Iterator<Item = I> + '_ {
    [
        (x - 1, y),
        (x + 1, y),
        (x, y - 1),
        (x, y + 1),
        (x + 1, y + 1),
        (x + 1, y - 1),
        (x - 1, y + 1),
        (x - 1, y - 1),
    ]
    .into_iter()
    .flat_map(move |p| get(&input, p))
}

fn basin_neighbours(input: &[Vec<I>], (x, y): (I, I)) -> impl Iterator<Item = (I, I)> + '_ {
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
        .into_iter()
        .flat_map(move |p| get(&input, p).and_then(|v| if v < 9 { Some(p) } else { None }))
}

fn basin_size(input: &[Vec<I>], (x, y): (I, I)) -> usize {
    // HashSet seems to be slower here
    let mut res = FxHashSet::default();
    let mut todo = FxHashSet::default();
    todo.insert((x, y));
    while !todo.is_empty() {
        let next = *todo.iter().next().unwrap();
        todo.remove(&next);
        res.insert(next);
        for p in basin_neighbours(&input, next) {
            if !res.contains(&p) {
                todo.insert(p);
            }
        }
    }
    res.len()
}

pub fn solve(input: &mut dyn BufRead, verify_expected: bool, output: bool) -> Duration {
    let input: Vec<Vec<I>> = input
        .lines()
        .map(|s| {
            s.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as I)
                .collect()
        })
        .collect();
    let s = Instant::now();

    let mut lows = vec![];
    let mut basin_starts = vec![];

    for r in 0..input.len() {
        for c in 0..input[r].len() {
            let pos = (r as I, c as I);
            let value = get(&input, pos).unwrap();
            if around(&input, pos).all(|v| v > value) {
                lows.push(value);
                basin_starts.push(pos);
            }
        }
    }

    let part1: I = lows.iter().sum::<I>() + lows.len() as I;

    let mut basin_sizes: Vec<_> = basin_starts
        .into_iter()
        .map(|p| basin_size(&input, p))
        .collect();
    let (top, _, _) = basin_sizes.select_nth_unstable_by_key(3, |v| Reverse(*v));
    let part2 = top.iter().product::<usize>();

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(575, part1);
        assert_eq!(1019700, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    e
}
