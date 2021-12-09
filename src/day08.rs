use maplit::{btreeset, hashmap};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::BTreeSet;
use std::io::BufRead;
use std::time::{Duration, Instant};

fn solve_part1(lines: &[String]) -> usize {
    let mut c = 0;
    for line in lines {
        let line: Vec<_> = line.split(' ').collect();
        let out = &line[11..];

        c += out
            .iter()
            .filter(|p| p.len() == 2 || p.len() == 3 || p.len() == 4 || p.len() == 7)
            .count()
    }
    c
}

fn conv(s: &str) -> BTreeSet<char> {
    s.chars().collect()
}

fn trans(
    input: &BTreeSet<char>,
    map: &FxHashMap<char, FxHashSet<char>>,
    chars_to_digit: &FxHashMap<BTreeSet<char>, usize>,
) -> usize {
    debug_assert!(map.iter().all(|v| v.1.len() == 1));
    let translated: BTreeSet<char> = input
        .iter()
        .map(|c| *map[c].iter().next().unwrap())
        .collect();
    let ret = chars_to_digit[&translated];
    ret
}

fn solve_part2(lines: &[String]) -> usize {
    let mut known_uniq_lens = FxHashMap::default();
    known_uniq_lens.insert(2, 1);
    known_uniq_lens.insert(4, 4);
    known_uniq_lens.insert(3, 7);
    known_uniq_lens.insert(7, 8);
    let digit_to_chars: FxHashMap<_, FxHashSet<_>> = hashmap! {
        0 => btreeset!{'a','b','c','e','f','g'}.into_iter().collect(),
        1 => btreeset!{'c','f'}.into_iter().collect(),
        2 => btreeset!{'a','c','d','e','g'}.into_iter().collect(),
        3 => btreeset!{'a','c','d','f','g'}.into_iter().collect(),
        4 => btreeset!{'b','c','d','f'}.into_iter().collect(),
        5 => btreeset!{'a','b','d','f','g'}.into_iter().collect(),
        6 => btreeset!{'a','b','d','e','f','g'}.into_iter().collect(),
        7 => btreeset!{'a','c','f'}.into_iter().collect(),
        8 => btreeset!{'a','b','c','d','e','f','g'}.into_iter().collect(),
        9 => btreeset!{'a','b','c','d','f','g'}.into_iter().collect(),
    }
    .into_iter()
    .collect();
    let chars_to_digit: FxHashMap<BTreeSet<char>, usize> = digit_to_chars
        .iter()
        .map(|(d, c)| (c.iter().cloned().collect(), *d))
        .collect();
    let mut chars_to_count: FxHashMap<char, usize> = FxHashMap::default();
    for (_, chars) in &digit_to_chars {
        for c in chars {
            *chars_to_count.entry(*c).or_default() += 1;
        }
    }
    let mut count_to_chars: FxHashMap<usize, FxHashSet<char>> = FxHashMap::default();
    for (c, count) in &chars_to_count {
        count_to_chars.entry(*count).or_default().insert(*c);
    }
    let mut cands: FxHashMap<_, FxHashSet<_>> = FxHashMap::default();
    cands.insert(
        'a',
        btreeset! {'a','b','c','d','e','f','g'}
            .into_iter()
            .collect(),
    );
    cands.insert(
        'b',
        btreeset! {'a','b','c','d','e','f','g'}
            .into_iter()
            .collect(),
    );
    cands.insert(
        'c',
        btreeset! {'a','b','c','d','e','f','g'}
            .into_iter()
            .collect(),
    );
    cands.insert(
        'd',
        btreeset! {'a','b','c','d','e','f','g'}
            .into_iter()
            .collect(),
    );
    cands.insert(
        'e',
        btreeset! {'a','b','c','d','e','f','g'}
            .into_iter()
            .collect(),
    );
    cands.insert(
        'f',
        btreeset! {'a','b','c','d','e','f','g'}
            .into_iter()
            .collect(),
    );
    cands.insert(
        'g',
        btreeset! {'a','b','c','d','e','f','g'}
            .into_iter()
            .collect(),
    );
    let mut c = 0;
    for line in lines {
        let mut line: Vec<BTreeSet<char>> = line.split(' ').map(|s| conv(s)).collect();
        line.remove(10);
        let pats = &line[0..10];
        let out = &line[10..];
        let mut cands = cands.clone();
        let mut counts_of_chars: FxHashMap<char, usize> = FxHashMap::default();
        for pat in pats {
            for c in pat {
                *counts_of_chars.entry(*c).or_default() += 1;
            }
        }
        for (c, count) in &counts_of_chars {
            let candidate = &count_to_chars[&count];
            if candidate.len() == 1 {
                let s: char = *candidate.iter().next().unwrap();
                for (_, v) in cands.iter_mut() {
                    v.remove(&s);
                }
                cands.insert(*c, candidate.clone());
            }
        }
        for pat in pats {
            if let Some(digit) = known_uniq_lens.get(&pat.len()) {
                let wires = &digit_to_chars[digit];
                for c in pat {
                    let old = &cands[c];
                    let new: FxHashSet<_> = old.intersection(wires).cloned().collect();
                    if new.len() == 1 {
                        let s = new.iter().next().unwrap();
                        for (_, tmp) in cands.iter_mut() {
                            tmp.remove(s);
                        }
                    }
                    cands.insert(*c, new);
                }
            }
        }
        let singletons: FxHashSet<_> = cands
            .iter()
            .filter(|p| p.1.len() == 1)
            .flat_map(|p| p.1.clone())
            .collect();

        cands
            .iter_mut()
            .filter(|tmp| tmp.1.len() > 1)
            .for_each(|tmp| *tmp.1 = tmp.1.difference(&singletons).cloned().collect());

        c += out
            .iter()
            .fold(0, |acc, p| 10 * acc + trans(p, &cands, &chars_to_digit));
    }
    c
}
pub fn solve(input: &mut dyn BufRead, verify_expected: bool, output: bool) -> Duration {
    let input: Vec<String> = input.lines().map(|s| s.unwrap()).collect();
    let s = Instant::now();

    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);

    let e = s.elapsed();
    if verify_expected {
        assert_eq!(476, part1);
        assert_eq!(1011823, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    e
}
