use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::time::Duration;
use structopt::StructOpt;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

#[derive(StructOpt, Debug)]
#[structopt(author)]
struct Opt {
    #[structopt(short, long)]
    skip_verification: bool,

    #[structopt(short, long)]
    day_to_run: Option<usize>,

    #[structopt(short, long)]
    input_file: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();

    let solutions: Vec<&dyn Fn(&mut dyn BufRead, bool) -> Duration> = vec![
        &day01::solve,
        &day02::solve,
        &day03::solve,
        &day04::solve,
        &day05::solve,
    ];

    let mut total = Duration::from_secs(0);
    for (i, solution) in solutions.iter().enumerate() {
        if Some(i + 1) == opt.day_to_run || opt.day_to_run.is_none() {
            let mut input = match &opt.input_file {
                None => BufReader::new(File::open(format!("inputs/day{:02}", i + 1)).unwrap()),
                Some(path) => BufReader::new(File::open(path).unwrap()),
            };
            let t = solution(&mut input, !opt.skip_verification);
            println!("Day {:02} took {:?} to compute", i + 1, t);
            total += t;
        }
    }
    println!(
        "\nTotal time: {:?} (avg per day {:?})",
        total,
        total.div_f64(solutions.len() as f64)
    );
}
