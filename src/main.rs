mod day01;
mod day02;
mod day03;
mod day04;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;

fn main() {
    let verify_expected = true;
    let solutions: Vec<&dyn Fn(&mut dyn BufRead, bool) -> Duration> =
        vec![&day01::solve, &day02::solve, &day03::solve, &day04::solve];

    let mut total = Duration::from_secs(0);
    for (i, solution) in solutions.iter().enumerate() {
        let t = solution(
            &mut BufReader::new(File::open(format!("inputs/day{:02}", i + 1)).unwrap()),
            verify_expected,
        );
        println!("Day {:02} took {:?} to compute", i + 1, t);
        total += t;
    }
    println!("Total time: {:?} ({:?})", total, total.div_f64(solutions.len() as f64));
}
