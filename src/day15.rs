use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::BufRead;
use std::time::{Duration, Instant};

type P = (isize, isize);
type V<T> = smallvec::SmallVec<[T; 4]>;

fn get(map: &[Vec<u32>], (row, col): P) -> Option<u32> {
    if row < 0 || col < 0 || row >= map.len() as isize || col >= map[row as usize].len() as isize {
        None
    } else {
        Some(map[row as usize][col as usize])
    }
}

fn neighbours(map: &[Vec<u32>], (r, c): P) -> V<(P, u32)> {
    let mut v = V::new();
    if let Some(cost) = get(map, (r - 1, c)) {
        v.push(((r - 1, c), cost));
    }
    if let Some(cost) = get(map, (r + 1, c)) {
        v.push(((r + 1, c), cost));
    }
    if let Some(cost) = get(map, (r, c - 1)) {
        v.push(((r, c - 1), cost));
    }
    if let Some(cost) = get(map, (r, c + 1)) {
        v.push(((r, c + 1), cost));
    }
    debug_assert!(!v.spilled());
    v
}

fn make_bigger(map: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let mut all = vec![];
    let mut rows = vec![];
    for row in map {
        let mut new_row = vec![];
        for offset in 0..5 {
            for c in row {
                let mut v = *c + offset;
                if v > 9 {
                    v -= 9;
                }
                new_row.push(v);
            }
        }
        rows.push(new_row);
    }
    for offset in 0..5 {
        for row in &rows {
            let mut new_row = row.clone();
            for v in &mut new_row {
                let mut new_v = *v + offset;
                if new_v > 9 {
                    new_v -= 9;
                }
                *v = new_v;
            }
            all.push(new_row);
        }
    }
    all
}

fn find(input: &[Vec<u32>]) -> u32 {
    // A* but turns out zero estimate/heuristic is faster...
    let end = (input.len() - 1) as isize;
    let end = (end, end);
    let mut lowest_risk_to: FxHashMap<P, u32> = FxHashMap::default();
    lowest_risk_to.insert((0, 0), 0);
    let mut todo = BinaryHeap::new();
    todo.push((Reverse((0) as u32), 0, (0, 0)));
    while let Some((_, cost, node)) = todo.pop() {
        if node == end {
            return cost;
        }
        for (neighbour, neighbour_cost) in neighbours(input, node) {
            let new_cost = cost + neighbour_cost;
            if new_cost < *lowest_risk_to.get(&neighbour).unwrap_or(&u32::max_value()) {
                lowest_risk_to.insert(neighbour, new_cost);
                let estimate = new_cost; // slower: "+ (end.0 - neighbour.0 + end.1 - neighbour.1) as u32;"
                todo.push((Reverse(estimate), new_cost, neighbour));
            }
        }
    }
    unreachable!()
}

pub fn solve(input: &mut dyn BufRead, verify_expected: bool, output: bool) -> Duration {
    let input: Vec<Vec<u32>> = input
        .lines()
        .map(|s| {
            s.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let s = Instant::now();

    let part1 = find(&input);

    let big = make_bigger(&input);
    for row in &big {
        for v in row {
            assert!(*v < 10);
            assert!(*v > 0);
        }
    }

    let part2 = find(&big);

    let e = s.elapsed();
    if verify_expected {
        assert_eq!(435, part1);
        assert_eq!(2842, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    e
}

#[allow(dead_code)]
fn dump(map: &[Vec<u32>]) {
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            print!("{}", map[row][col]);
        }
        println!()
    }
}

#[allow(dead_code)]
fn dump_path(map: &[Vec<u32>], points: &FxHashSet<P>) {
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if points.contains(&(row as isize, col as isize)) {
                print!("{}", map[row][col]);
            } else {
                print!(" ");
            }
        }
        println!()
    }
}
