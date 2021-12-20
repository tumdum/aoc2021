use criterion::{criterion_group, criterion_main, Criterion};
use std::io::BufReader;

macro_rules! benchmark {
    ($name: ident) => {
        fn $name(c: &mut Criterion) {
            let input = std::fs::read(format!(
                "{}/inputs/{}",
                env!("CARGO_MANIFEST_DIR"),
                stringify!($name)
            ))
            .unwrap();
            c.bench_function(stringify!($name), |b| {
                b.iter(|| aoc21::$name::solve(&mut BufReader::new(&input[..]), false, false))
            });
        }
    };
}

macro_rules! benchmarks {
    ($($name:ident),+) => {
        $(
            benchmark!{$name}
        )+

        criterion_group!(benches, $($name,)+);
        criterion_main!(benches);
    }
}

benchmarks! {day01,day02,day03,day04,day05,day06,day07,day08,day09,day10,day11,day12,day13,day14,day15,day16,day17,day18,day19,day20}
