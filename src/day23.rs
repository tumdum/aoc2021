use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::BTreeMap;
use std::io::BufRead;
use std::time::{Duration, Instant};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

type P = (i8, i8);

fn around((x, y): (i8, i8)) -> [P; 4] {
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}

fn in_front_of_room(p: P) -> bool {
    p == (2, 1) || p == (4, 1) || p == (6, 1) || p == (8, 1)
}

fn can_move_to(pos: P, map: &FxHashSet<P>, others: &BTreeMap<P, char>) -> Vec<P> {
    around(pos)
        .into_iter()
        .filter(|c| !others.contains_key(&c) && map.contains(&c))
        .collect()
}

fn is_room(p: P) -> bool {
    let rooms = [
        (2, 2),
        (2, 3),
        (2, 4),
        (2, 5),
        (4, 2),
        (4, 3),
        (4, 4),
        (4, 5),
        (6, 2),
        (6, 3),
        (6, 4),
        (6, 5),
        (8, 2),
        (8, 3),
        (8, 4),
        (8, 5),
    ];
    rooms.contains(&p)
}

fn is_target_room(p: P, c: char) -> bool {
    if c == 'A' {
        return p == (2, 3) || p == (2, 2) || p == (2,4) || p == (2,5)
    }

    if c == 'B' {
        return p == (4, 3) || p == (4, 2) || p == (4,4) || p == (4,5)
    }

    if c == 'C' {
        return p == (6, 3) || p == (6, 2)  || p == (6, 4) || p == (6,5)
    }

    if c == 'D' {
        return p == (8, 3) || p == (8, 2) || p == (8,4) || p == (8,5)
    }
    panic!("unknown letter");
}

fn reachable_from(start: P, map: &FxHashSet<P>, state: &BTreeMap<P, char>) -> Vec<(P,u64)> {
    let mut seen: FxHashSet<P> = FxHashSet::default();
    let mut ret: FxHashSet<(P,u64)> = FxHashSet::default();
    let mut todo: FxHashSet<(P,u64)> = FxHashSet::default();
    todo.insert((start,0));

    while !todo.is_empty() {
        let e = *todo.iter().next().unwrap();
        todo.remove(&e);
        let (next, dist) = e;
        seen.insert(next);
        if next != start {
            ret.insert((next, dist));
        }
        for c in can_move_to(next, map, state) {
            if !seen.contains(&c) {
                todo.insert((c,dist+1));
            }
        }
    }
    // println!("ret: {:?}", ret);

    let color: char = *state.get(&start).unwrap();
    let start_in_room = is_room(start);
    let mut real_ret = vec![];
    for (end,dist) in ret {
        if in_front_of_room(end) {
            continue;
        }
        let end_in_room = is_room(end);
        let end_is_target = is_target_room(end, color);
        if start_in_room && !end_in_room {
            real_ret.push((end, dist));
            continue;
        }
        if !start_in_room && end_in_room && end_is_target {
            let room = end.0;
            if [2,3,4,5].into_iter().flat_map(|y| state.get(&(room, y))).all(|c| *c == color) {
                real_ret.push((end, dist));
                continue;
            }
        }
    }

    real_ret.sort();
    real_ret
}

fn print(m: &FxHashSet<P>, others: &BTreeMap<P, char>) {
    for y in 1..=5 {
        for x in 0..=10 {
            if m.contains(&(x, y)) {
                if let Some(c) = others.get(&(x, y)) {
                    print!("{}", c);
                } else {
                    print!(".");
                }
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn move_cost(dist: u64, color: char) -> u64 {
    match color {
        'A' => dist,
        'B' => 10*dist,
        'C' => 100*dist,
        'D' => 1000*dist,
        _ => panic!("unknown color")
    }
}

pub fn solve(input: &mut dyn BufRead, verify_expected: bool, output: bool) -> Duration {
    // let input: Vec<i64> = input.lines().map(|s| s.unwrap().parse().unwrap()).collect();

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
        // /*
        (2, 5),
        (2, 4),
        // */
        (2, 3),
        (2, 2),
        // /*
        (4, 5),
        (4, 4),
        // */
        (4, 3),
        (4, 2),
        // /*
        (6, 5),
        (6, 4),
        // */
        (6, 3),
        (6, 2),
        // /*
        (8, 5),
        (8, 4),
        // */
        (8, 3),
        (8, 2),
    ]
    .into_iter()
    .collect();

    let others: BTreeMap<P, char> = [
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
        // ((2, 3), 'A'),
        // /*
        ((2, 3), 'A'),
        ((2, 4), 'A'),
        ((2, 5), 'A'),
        // */
        ((4, 2), 'B'),
        // ((4, 3), 'B'),
        // /*
        ((4, 3), 'B'),
        ((4, 4), 'B'),
        ((4, 5), 'B'),
        // */
        ((6, 2), 'C'),
        // ((6, 3), 'C'),
        // /*
        ((6, 3), 'C'),
        ((6, 4), 'C'),
        ((6, 5), 'C'),
        // */
        ((8, 2), 'D'),
        // ((8, 3), 'D'),
        // /*
        ((8, 3), 'D'),
        ((8, 4), 'D'),
        ((8, 5), 'D'),
        // */
    ]
    .into_iter()
    .collect();


    print(&map, &others);
    println!("end:");
    print(&map, &end);


    let mut todo: BinaryHeap<(Reverse<u64>, BTreeMap<P, char>)> = BinaryHeap::new();
    let mut seen = FxHashMap::default();
    todo.push((Reverse(0), others.clone()));
    seen.insert(others, 0);

    let mut part2 = 0;
    let mut c = 0;
    while let Some((Reverse(cost), state)) = todo.pop() {
        c += 1;
        if c % 100000 == 0 {
            println!("todo: {}, current cost: {}", todo.len(), cost);
        }
        if state == end {
            part2 = cost;
            break;
        }
        for (pos, _) in &state {
            for (target, dist) in reachable_from(*pos, &map, &state) {
                let mut new_state = state.clone();
                let c = new_state.remove(&pos).unwrap();
                new_state.insert(target, c);
                let new_cost = cost + move_cost(dist, c);
                if !seen.contains_key(&new_state) {
                    seen.insert(new_state.clone(), new_cost);
                    todo.push((Reverse(new_cost), new_state));
                } else {
                    let old_cost = seen.get(&new_state).unwrap();
                    if *old_cost > new_cost {
                        seen.insert(new_state.clone(), new_cost);
                        todo.push((Reverse(new_cost), new_state));
                    }
                }
            }
        }
    }
    let e = s.elapsed();
    if verify_expected {
        // assert_eq!(1602, part1);
        assert_eq!(46772, part2);
    }
    if output {
        // println!("\t{}", part1);
        println!("\t{}", part2);
    }
    e
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_cost_tests() {
        // assert_eq!(40, move_cost((6,2),(3,1),'B'));
    }

}
