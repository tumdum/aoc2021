use rustc_hash::FxHashMap;
use std::io::BufRead;
use std::time::{Duration, Instant};

fn step(
    s: &FxHashMap<(char, char), usize>,
    rules: &FxHashMap<(char, char), char>,
) -> FxHashMap<(char, char), usize> {
    let mut ret = FxHashMap::default();
    for (rule, ins) in rules {
        if let Some(count) = s.get(rule) {
            *ret.entry((rule.0, *ins)).or_default() += count;
            *ret.entry((*ins, rule.1)).or_default() += count;
        }
    }
    ret
}

fn answer(s: &FxHashMap<(char, char), usize>, (first, last): (char, char)) -> usize {
    let mut hist = make_hist(s);
    *hist.entry(first).or_default() += 1;
    *hist.entry(last).or_default() += 1;
    // Each char is counted twice - two different pairs cantain it
    (hist.values().max().unwrap() - hist.values().min().unwrap()) / 2
}

fn make_hist(s: &FxHashMap<(char, char), usize>) -> FxHashMap<char, usize> {
    let mut hist: FxHashMap<char, usize> = FxHashMap::default();
    for ((a, b), count) in s {
        *hist.entry(*a).or_default() += count;
        *hist.entry(*b).or_default() += count;
    }
    hist
}

pub fn solve(input: &mut dyn BufRead, verify_expected: bool, output: bool) -> Duration {
    let input: Vec<String> = input.lines().map(|s| s.unwrap()).collect();

    let rules = input[2..]
        .iter()
        .map(|s| {
            let mut s = s.split(" -> ");
            let from: Vec<_> = s.next().unwrap().chars().take(2).collect();
            let to = s.next().unwrap();
            ((from[0], from[1]), to.chars().next().unwrap())
        })
        .collect();

    let s = Instant::now();

    let chars: Vec<_> = input[0].chars().collect();
    let mut template = FxHashMap::default();
    for w in chars.windows(2) {
        *template.entry((w[0], w[1])).or_default() += 1;
    }
    let edges = (*chars.first().unwrap(), *chars.last().unwrap());

    let mut part1 = 0;
    for s in 1..=40 {
        template = step(&template, &rules);
        if s == 10 {
            part1 = answer(&template, edges);
        }
    }
    let part2 = answer(&template, edges);

    let e = s.elapsed();
    if verify_expected {
        assert_eq!(2745, part1);
        assert_eq!(3420801168962, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    e
}
