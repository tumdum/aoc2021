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

benchmark! {day01}
benchmark! {day02}
benchmark! {day03}
benchmark! {day04}
benchmark! {day05}

criterion_group!(benches, day01, day02, day03, day04, day05);
criterion_main!(benches);
