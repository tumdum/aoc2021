use regex::Regex;
use std::io::BufRead;
use std::ops::RangeInclusive;
use std::time::{Duration, Instant};

type P = (i64, i64);

fn step(p: P, v: P) -> (P, P) {
    let mut new_p = p;
    new_p.0 += v.0;
    new_p.1 += v.1;

    let mut new_v = v;
    if new_v.0 > 0 {
        new_v.0 -= 1
    } else if new_v.0 < 0 {
        new_v.0 += 1
    }
    new_v.1 -= 1;
    (new_p, new_v)
}

fn is_in(p: P, x: &RangeInclusive<i64>, y: &RangeInclusive<i64>) -> bool {
    x.contains(&p.0) && y.contains(&p.1)
}

fn find_max_h(
    mut p: P,
    mut v: P,
    target_x: &RangeInclusive<i64>,
    target_y: &RangeInclusive<i64>,
    steps: i64,
) -> Option<i64> {
    let start = p;
    let mut max_h = None;
    for _ in 0..steps {
        // below
        if p.1 < *target_y.end().min(target_y.start()) {
            break;
        }
        // after
        if p.0 > *target_x.start().max(target_x.end()) {
            break;
        }

        if p.1 > max_h.unwrap_or(0) {
            max_h = Some(p.1)
        }
        if is_in(p, target_x, target_y) {
            if max_h.is_none() {
                max_h = Some(start.1)
            }
            return max_h;
        }
        let (np, nv) = step(p, v);
        p = np;
        v = nv;

        if v.0 == 0 && !target_x.contains(&p.0) {
            break;
        }
    }
    None
}

fn find_candidate_x(mut x: i64, target_x: &RangeInclusive<i64>) -> bool {
    let mut p = (0, 0);
    let max = *target_x.start().max(target_x.end());

    loop {
        if p.0 > max {
            break;
        }

        if target_x.contains(&p.0) {
            return true;
        }

        let (np, nv) = step(p, (x, 0));
        p = np;
        x = nv.0;

        if x == 0 {
            break;
        }
    }
    false
}

pub fn solve(input: &mut dyn BufRead, verify_expected: bool, output: bool) -> Duration {
    let re = Regex::new(r#"target area: x=(.+)\.\.(.+), y=(.+)\.\.(.+)"#).unwrap();
    let input = input.lines().map(|s| s.unwrap()).next().unwrap();
    let cap = re.captures(&input).unwrap();
    let target_x: RangeInclusive<i64> = (cap[1].parse().unwrap())..=(cap[2].parse().unwrap());
    let target_y: RangeInclusive<i64> = (cap[3].parse().unwrap())..=(cap[4].parse().unwrap());

    let s = Instant::now();

    let steps = target_x
        .start()
        .abs()
        .max(target_x.end().abs())
        .max(target_y.start().abs())
        .max(target_y.end().abs());

    let max = target_x
        .start()
        .abs()
        .max(target_x.end().abs())
        .max(target_y.start().abs())
        .max(target_y.end().abs());

    let y_bound: i64 = target_y.start().abs().max(target_y.end().abs());

    let xs = (0..=max)
        .filter(|x| find_candidate_x(*x, &target_x))
        .collect::<Vec<_>>();

    let mut heights = vec![];
    for x in xs {
        for y in -y_bound..=y_bound {
            if let Some(h) = find_max_h((0, 0), (x, y), &target_x, &target_y, steps) {
                heights.push(h);
            }
        }
    }

    let part1 = *heights.iter().max().unwrap();
    let part2 = heights.len();

    let e = s.elapsed();
    if verify_expected {
        assert_eq!(3160, part1);
        assert_eq!(1928, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    e
}
