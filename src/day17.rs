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

pub fn solve(_: &mut dyn BufRead, verify_expected: bool, output: bool) -> Duration {
    let s = Instant::now();

    // target area: x=282..314, y=-80..-45
    let target_x = 282i64..=314;
    let target_y = -80i64..=-45;

    let steps = target_x
        .start()
        .abs()
        .max(target_x.end().abs())
        .max(target_y.start().abs())
        .max(target_y.end().abs());

    let mut heights = vec![];
    for x in 20..=314 {
        for y in -80..80 {
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
