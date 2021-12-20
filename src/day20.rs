use rustc_hash::FxHashMap;
use std::io::BufRead;
use std::time::{Duration, Instant};
use itertools::iproduct;

fn get_new_pixel(
    (row, col): (i32, i32),
    map: &FxHashMap<(i32, i32), bool>,
    algo: &[bool],
    default: bool,
) -> bool {
    let mut acc = 0;
    let mut i = 0;
    for row_d in [-1i8, 0, 1] {
        for col_d in [-1i8, 0, 1] {
            let b = *map
                .get(&(row + row_d as i32, col + col_d as i32))
                .unwrap_or(&default);
            acc = acc | (1 << (9 - i - 1)) * (b as usize);
            i += 1;
        }
    }
    algo[acc]
}

fn step(
    img: &FxHashMap<(i32, i32), bool>,
    algo: &[bool],
    default: bool,
    mut row_min: i32,
    mut row_max: i32,
    mut col_min: i32,
    mut col_max: i32,
) -> (FxHashMap<(i32, i32), bool>, (i32, i32, i32, i32)) {
    let mut ret = FxHashMap::default();
    row_min -= 1;
    col_min -= 1;
    row_max += 1;
    col_max += 1;
    for row in row_min..=row_max {
        for col in col_min..=col_max {
            ret.insert((row, col), get_new_pixel((row, col), img, algo, default));
        }
    }
    (ret, (row_min, row_max, col_min, col_max))
}

fn count(
    m: &FxHashMap<(i32, i32), bool>,
    row_min: i32,
    row_max: i32,
    col_min: i32,
    col_max: i32,
) -> usize {
    iproduct!(row_min..=row_max, col_min..=col_max).flat_map(|p| m.get(&p)).filter(|b| **b).count()
}

pub fn solve(input: &mut dyn BufRead, verify_expected: bool, output: bool) -> Duration {
    let input: Vec<String> = input.lines().map(|s| s.unwrap()).collect();

    let algo: Vec<bool> = input[0]
        .chars()
        .map(|c| if c == '.' { false } else { true })
        .collect();

    let input = &input[2..];

    let mut map = FxHashMap::default();

    let mut row_min = 0;
    let mut row_max = 0;
    let mut col_min = 0;
    let mut col_max = 0;

    for row in 0..input.len() {
        row_min = row_min.min(row as i32);
        row_max = row_max.max(row as i32);
        for col in 0..input[row].len() {
            col_min = col_min.min(col as i32);
            col_max = col_max.max(col as i32);
            map.insert(
                (row as i32, col as i32),
                if input[row].chars().nth(col).unwrap() == '.' {
                    false
                } else {
                    true
                },
            );
        }
    }

    let s = Instant::now();

    let mut default = false;
    let mut part1 = 0;
    let mut part2 = 0;

    for i in 1..=50 {
        let (m, (r_min, r_max, c_min, c_max)) =
            step(&map, &algo, default, row_min, row_max, col_min, col_max);

        map = m;
        row_min = r_min;
        row_max = r_max;
        col_min = c_min;
        col_max = c_max;
        part2 = count(&map, row_min, row_max, col_min, col_max);
        if i == 2 {
            part1 = part2;
        }
        default = !default;
    }

    let e = s.elapsed();
    if verify_expected {
        assert_eq!(5573, part1);
        assert_eq!(20097, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    e
}
