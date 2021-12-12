use rustc_hash::FxHashMap;
use std::io::BufRead;
use std::time::{Duration, Instant};

type V<T> = smallvec::SmallVec<[T; 6]>;
type C = Cave;
type G = smallvec::SmallVec<[V<C>; 12]>;

const END_ID: u8 = 0;
const START_ID: u8 = 1;
const END: Cave = Cave::small(END_ID);
const START: Cave = Cave::small(START_ID);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Cave(u8);

impl Cave {
    const fn small(v: u8) -> Self {
        Cave(0b1000_0000 | v)
    }
    const fn big(v: u8) -> Self {
        Cave(v)
    }
    const fn idx(self) -> usize {
        (self.0 & 0b0111_1111) as usize
    }
}

impl Cave {
    const fn is_small(self) -> bool {
        self.0 & 0b1000_0000 != 0
    }
}

fn paths_to(target: &C, avoid: &VisitTracker, g: &G) -> usize {
    let mut c = 0;
    for other in &g[target.idx()] {
        if *other == START {
            c += 1;
        } else if avoid.can_visit(*other) {
            if other.is_small() {
                let mut new_avoid = avoid.clone();
                new_avoid.add(*other);
                c += paths_to(other, &new_avoid, g);
            } else {
                c += paths_to(other, &avoid, g);
            }
        }
    }
    c
}

fn paths_to2(target: C, tracker: &VisitTracker, g: &G) -> usize {
    let mut c = 0;
    for other in &g[target.idx()] {
        if *other == START {
            c += 1;
        } else if tracker.can_visit2(*other) {
            if other.is_small() {
                let mut new_tracker: VisitTracker = tracker.clone();
                new_tracker.add2(*other);
                c += paths_to2(*other, &new_tracker, g);
            } else {
                c += paths_to2(*other, tracker, g);
            }
        }
    }
    c
}

#[derive(Debug, Clone)]
struct VisitTracker {
    counts: [u8; 15],
    visited_twice: bool,
}

impl VisitTracker {
    fn new() -> Self {
        let mut counts = [0; 15];
        counts[END.idx()] = 2;
        Self {
            counts,
            visited_twice: false,
        }
    }

    fn add(&mut self, s: C) {
        self.counts[s.idx()] = 1;
    }

    const fn can_visit(&self, s: C) -> bool {
        self.counts[s.idx()] == 0
    }

    const fn can_visit2(&self, s: C) -> bool {
        let c = self.counts[s.idx()];
        c == 0 || (c == 1 && !self.visited_twice)
    }

    fn add2(&mut self, s: C) {
        let idx = s.idx();
        if self.counts[idx] == 1 {
            self.counts[idx] = 2;
            self.visited_twice = true;
        } else {
            self.counts[idx] = 1;
        }
    }
}

pub fn solve(input: &mut dyn BufRead, verify_expected: bool, output: bool) -> Duration {
    let input: Vec<(String, String)> = input
        .lines()
        .map(|s| {
            let s = s.unwrap();
            let mut s = s.split('-');
            let a = s.next().unwrap().to_owned();
            let b = s.next().unwrap().to_owned();
            (a, b)
        })
        .collect();

    let s = Instant::now();

    let mut names: FxHashMap<String, C> = FxHashMap::default();
    names.insert("end".to_owned(), END);
    names.insert("start".to_owned(), START);
    let mut next_name = 2;
    for (a, b) in &input {
        if !names.contains_key(a) {
            if a.chars().all(|c| c.is_lowercase()) {
                names.insert(a.to_owned(), C::small(next_name));
            } else {
                names.insert(a.to_owned(), C::big(next_name));
            }
            next_name += 1;
        }
        if !names.contains_key(b) {
            if b.chars().all(|c| c.is_lowercase()) {
                names.insert(b.to_owned(), C::small(next_name));
            } else {
                names.insert(b.to_owned(), C::big(next_name));
            }
            next_name += 1;
        }
    }

    let mut g: G = G::from_elem(V::new(), 12);
    debug_assert!(!g.spilled());

    for (a, b) in input {
        let a = names[&a];
        let b = names[&b];
        g[a.idx()].push(b);
        g[b.idx()].push(a);
    }

    let avoid = VisitTracker::new();
    let part1 = paths_to(&END, &avoid, &g);

    let avoid = VisitTracker::new();
    let part2 = paths_to2(END, &avoid, &g);

    let e = s.elapsed();
    if verify_expected {
        assert_eq!(5228, part1);
        assert_eq!(131228, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    e
}
