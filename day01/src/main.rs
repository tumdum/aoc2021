use std::io::{stdin, BufRead};
fn main() {
    let input: Vec<i64> = stdin()
        .lock()
        .lines()
        .map(|s| s.unwrap().parse().unwrap())
        .collect();

    println!("{}", input.windows(2).filter(|w| w[0] < w[1]).count());
    let input: Vec<i64> = input.windows(3).map(|w| w.iter().sum()).collect();
    println!("{}", input.windows(2).filter(|w| w[0] < w[1]).count());
}
