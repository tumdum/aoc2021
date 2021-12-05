use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn day01(c: &mut Criterion) {
    let input = include_bytes!("../inputs/day01");
    c.bench_function("day 01", |b| {
        b.iter(|| aoc21::day01::solve(&mut BufReader::new(&input[..]), false, false))
    });
}

fn day02(c: &mut Criterion) {
    let input = include_bytes!("../inputs/day02");
    c.bench_function("day 02", |b| {
        b.iter(|| aoc21::day02::solve(&mut BufReader::new(&input[..]), false, false))
    });
}

fn day03(c: &mut Criterion) {
    let input = include_bytes!("../inputs/day03");
    c.bench_function("day 03", |b| {
        b.iter(|| aoc21::day03::solve(&mut BufReader::new(&input[..]), false, false))
    });
}

fn day04(c: &mut Criterion) {
    let input = include_bytes!("../inputs/day04");
    c.bench_function("day 04", |b| {
        b.iter(|| aoc21::day04::solve(&mut BufReader::new(&input[..]), false, false))
    });
}

fn day05(c: &mut Criterion) {
    let input = include_bytes!("../inputs/day05");
    c.bench_function("day 05", |b| {
        b.iter(|| aoc21::day05::solve(&mut BufReader::new(&input[..]), false, false))
    });
}

criterion_group!(benches, day01, day02, day03, day04, day05);
criterion_main!(benches);
