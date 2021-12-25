use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};
use std::io::BufRead;
use std::time::{Duration, Instant};
use std::hash::{Hasher,Hash};

const SMALL_MAP: [(u8, u8); 19] = [
    (0, 1),
    (1, 1),
    (2, 1),
    (3, 1),
    (4, 1),
    (5, 1),
    (6, 1),
    (7, 1),
    (8, 1),
    (9, 1),
    (10, 1),
    (2, 3),
    (2, 2),
    (4, 3),
    (4, 2),
    (6, 3),
    (6, 2),
    (8, 3),
    (8, 2),
];

const MAP: [(u8, u8); 27] = [
    (0, 1),
    (1, 1),
    (2, 1),
    (3, 1),
    (4, 1),
    (5, 1),
    (6, 1),
    (7, 1),
    (8, 1),
    (9, 1),
    (10, 1),
    (2, 5),
    (2, 4),
    (2, 3),
    (2, 2),
    (4, 5),
    (4, 4),
    (4, 3),
    (4, 2),
    (6, 5),
    (6, 4),
    (6, 3),
    (6, 2),
    (8, 5),
    (8, 4),
    (8, 3),
    (8, 2),
];

const ROWS: usize = 7;
const COLS: usize = 12;

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
struct State {
    map: [u8; ROWS * COLS],
}

impl Default for State {
    fn default() -> Self {
        Self {
            map: [0; ROWS * COLS],
        }
    }
}

impl Hash for State {
    fn hash<H>(&self, hasher: &mut H) where H: Hasher {
        hasher.write(&self.map);
    }
}

impl State {
    fn is_invalid(&self, p: P) -> bool {
        p.0 < 0 || p.1 < 0
    }

    fn contains_key(&self, p: P) -> bool {
        if self.is_invalid(p) {
            return false;
        }
        self.map[p.1 as usize * COLS + p.0 as usize] != b'x'
    }

    fn get(&self, p: P) -> Option<char> {
        if self.map[p.1 as usize * COLS + p.0 as usize] != b'x' {
            Some(self.map[p.1 as usize * COLS + p.0 as usize] as char)
        } else {
            None
        }
    }
    fn remove(&mut self, p: P) -> Option<char> {
        let old = self.map[p.1 as usize * COLS + p.0 as usize];
        self.map[p.1 as usize * COLS + p.0 as usize] = b'x';
        if old != b'x' {
            Some(old as char)
        } else {
            None
        }
    }

    fn insert(&mut self, p: P, c: char) {
        self.map[p.1 as usize * COLS + p.0 as usize] = c as u8;
    }

    fn iter<'a>(&'a self, whole_map: &'static [(u8, u8)]) -> impl Iterator<Item = (P, char)> + 'a {
        whole_map.iter().flat_map(|(x, y)| {
            if self.map[*y as usize * COLS + *x as usize] != b'x' {
                Some((
                    (*x as i8, *y as i8),
                    self.map[*y as usize * COLS + *x as usize] as char,
                ))
            } else {
                None
            }
        })
    }

    fn new(other: &BTreeMap<P, char>) -> Self {
        let mut map = [b'x'; COLS * 7];
        for ((x, y), c) in other {
            map[*y as usize * COLS + *x as usize] = *c as u8;
        }
        Self { map }
    }

    fn new_hash(other: &FxHashSet<P>) -> Self {
        let mut map = [b'x'; COLS * 7];
        for (x, y) in other {
            map[*y as usize * COLS + *x as usize] = b'A';
        }
        Self { map }
    }
}

type P = (i8, i8);

fn around((x, y): (i8, i8), out: &mut [P; 4]) {
    out[0] = (x - 1, y);
    out[1] = (x + 1, y);
    out[2] = (x, y - 1);
    out[3] = (x, y + 1);
}

fn in_front_of_room(p: P) -> bool {
    p.1 == 1 && (p.0 == 2 || p.0 == 4 || p.0 == 6 || p.0 == 8)
}

fn is_room(p: P) -> bool {
    (p.0 == 2 || p.0 == 4 || p.0 == 6 || p.0 == 8) && p.1 > 1
}

fn is_target_room(p: P, c: char) -> bool {
    if c == 'A' {
        return p.0 == 2;
    }

    if c == 'B' {
        return p.0 == 4;
    }

    if c == 'C' {
        return p.0 == 6;
    }

    if c == 'D' {
        return p.0 == 8;
    }
    panic!("unknown letter");
}

type V = SmallVec<[(P, u32); 12]>;

fn reachable_from(start: P, map: &State, state: &State, bottom: i8) -> V {
    let mut seen = [false; ROWS * COLS];
    let mut ret = V::new();
    let mut todo = V::new();
    todo.push((start, 0));

    let mut tmp = [(0, 0); 4];

    while let Some((next, dist)) = todo.pop() {
        if next != start {
            if !in_front_of_room(next) {
                ret.push((next, dist));
            }
        }
        around(next, &mut tmp);
        for c in &tmp {
            if !state.contains_key(*c) && map.contains_key(*c) {
                if !seen[c.1 as usize * COLS + c.0 as usize] {
                    todo.push((*c, dist + 1));
                    seen[c.1 as usize * COLS + c.0 as usize] = true;
                }
            }
        }
    }

    let color: char = state.get(start).unwrap();
    let start_in_room = is_room(start);
    let mut real_ret = V::new();
    for (end, dist) in ret {
        let end_in_room = is_room(end);
        let end_is_target = is_target_room(end, color);
        if start_in_room && !end_in_room {
            real_ret.push((end, dist));
            continue;
        }
        if !start_in_room && end_in_room && end_is_target {
            let room = end.0;
            if (2..=bottom)
                .into_iter()
                .flat_map(|y| state.get((room, y)))
                .all(|c| c == color)
            {
                // once we enter the room, we go as far as possible
                let mut use_it = true;
                for y in end.1 + 1..=bottom {
                    if state.get((end.0, y)) == None {
                        use_it = false;
                        break;
                    }
                }
                if use_it {
                    real_ret.push((end, dist));
                    continue;
                }
            }
        }
    }

    real_ret
}

fn move_cost(dist: u32, color: char) -> u32 {
    match color {
        'A' => dist,
        'B' => 10 * dist,
        'C' => 100 * dist,
        'D' => 1000 * dist,
        _ => panic!("unknown color"),
    }
}

fn find_path(
    start: &State,
    end: &State,
    map: &FxHashSet<P>,
    whole_map: &'static [(u8, u8)],
    bottom: i8,
) -> u32 {
    let map = State::new_hash(&map);
    let mut todo: BinaryHeap<(Reverse<u32>, State)> = BinaryHeap::new();
    let mut seen = FxHashMap::default();
    todo.push((Reverse(0), start.clone()));
    seen.insert(start.clone(), 0);

    while let Some((Reverse(cost), state)) = todo.pop() {
        if &state == end {
            return cost;
        }
        for (pos, _) in state.iter(whole_map) {
            for (target, dist) in reachable_from(pos, &map, &state, bottom) {
                let mut new_state = state.clone();
                let c = new_state.remove(pos).unwrap();
                new_state.insert(target, c);
                let new_cost = cost + move_cost(dist, c);
                use std::collections::hash_map::Entry::*;
                match seen.entry(new_state.clone()) {
                    Occupied(mut e) => {
                        if *e.get() > new_cost {
                            e.insert(new_cost);
                            todo.push((Reverse(new_cost), new_state));
                        }
                    }
                    Vacant(e) => {
                        e.insert(new_cost);
                        todo.push((Reverse(new_cost), new_state));
                    }
                }
            }
        }
    }

    unreachable!()
}

pub fn solve(_input: &mut dyn BufRead, verify_expected: bool, output: bool) -> Duration {
    let s = Instant::now();

    /*

       01234567890
    1 #...........#
    2 ###D#C#A#B###
    3   #B#C#D#A#
         2345678
    */

    let map: FxHashSet<P> = [
        (0, 1),
        (1, 1),
        (2, 1),
        (3, 1),
        (4, 1),
        (5, 1),
        (6, 1),
        (7, 1),
        (8, 1),
        (9, 1),
        (10, 1),
        (2, 3),
        (2, 2),
        (4, 3),
        (4, 2),
        (6, 3),
        (6, 2),
        (8, 3),
        (8, 2),
    ]
    .into_iter()
    .collect();

    let start: BTreeMap<P, char> = [
        ((2, 2), 'D'),
        ((2, 3), 'B'),
        ((4, 2), 'C'),
        ((4, 3), 'C'),
        ((6, 2), 'A'),
        ((6, 3), 'D'),
        ((8, 2), 'B'),
        ((8, 3), 'A'),
    ]
    .into_iter()
    .collect();

    let end: BTreeMap<P, char> = [
        ((2, 2), 'A'),
        ((2, 3), 'A'),
        ((4, 2), 'B'),
        ((4, 3), 'B'),
        ((6, 2), 'C'),
        ((6, 3), 'C'),
        ((8, 2), 'D'),
        ((8, 3), 'D'),
    ]
    .into_iter()
    .collect();

    let start = State::new(&start);
    let end = State::new(&end);

    let part1 = find_path(&start, &end, &map, &SMALL_MAP, 3);

    let map: FxHashSet<P> = [
        (0, 1),
        (1, 1),
        (2, 1),
        (3, 1),
        (4, 1),
        (5, 1),
        (6, 1),
        (7, 1),
        (8, 1),
        (9, 1),
        (10, 1),
        (2, 5),
        (2, 4),
        (2, 3),
        (2, 2),
        (4, 5),
        (4, 4),
        (4, 3),
        (4, 2),
        (6, 5),
        (6, 4),
        (6, 3),
        (6, 2),
        (8, 5),
        (8, 4),
        (8, 3),
        (8, 2),
    ]
    .into_iter()
    .collect();

    let start: BTreeMap<P, char> = [
        ((2, 2), 'D'),
        ((2, 3), 'D'),
        ((2, 4), 'D'),
        ((2, 5), 'B'),
        ((4, 2), 'C'),
        ((4, 3), 'C'),
        ((4, 4), 'B'),
        ((4, 5), 'C'),
        ((6, 5), 'D'),
        ((6, 3), 'B'),
        ((6, 4), 'A'),
        ((6, 2), 'A'),
        ((8, 2), 'B'),
        ((8, 3), 'A'),
        ((8, 4), 'C'),
        ((8, 5), 'A'),
    ]
    .into_iter()
    .collect();

    let end: BTreeMap<P, char> = [
        ((2, 2), 'A'),
        ((2, 3), 'A'),
        ((2, 4), 'A'),
        ((2, 5), 'A'),
        ((4, 2), 'B'),
        ((4, 3), 'B'),
        ((4, 4), 'B'),
        ((4, 5), 'B'),
        ((6, 2), 'C'),
        ((6, 3), 'C'),
        ((6, 4), 'C'),
        ((6, 5), 'C'),
        ((8, 2), 'D'),
        ((8, 3), 'D'),
        ((8, 4), 'D'),
        ((8, 5), 'D'),
    ]
    .into_iter()
    .collect();

    let start = State::new(&start);
    let end = State::new(&end);

    let part2 = find_path(&start, &end, &map, &MAP, 5);

    let e = s.elapsed();
    if verify_expected {
        assert_eq!(15160, part1);
        assert_eq!(46772, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    e
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_cost_tests() {
        let mut s = State::default();
        s.insert((0, 1), 'A');
    }
}
