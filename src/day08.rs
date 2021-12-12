use maplit::{btreeset, hashmap};
use rustc_hash::{FxHashMap, FxHashSet};
use std::io::BufRead;
use std::time::{Duration, Instant};

type V = smallvec::SmallVec<[u8; 7]>;
const A: u8 = b'a';

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

fn conv(s: &str) -> V {
    let mut v: V = s.bytes().collect();
    debug_assert!(!v.spilled());
    v.sort_unstable();
    v
}

fn trans(input: &[u8], map: [u8; 7], chars_to_digit: &FxHashMap<V, u8>) -> usize {
    let mut translated: V = input.iter().map(|c| map[(c - A) as usize]).collect();
    translated.sort_unstable();
    chars_to_digit[&translated] as usize
}

fn solve_part2(lines: &[String]) -> usize {
    let mut known_uniq_lens = FxHashMap::default();
    known_uniq_lens.insert(2, 1);
    known_uniq_lens.insert(4, 4);
    known_uniq_lens.insert(3, 7);
    known_uniq_lens.insert(7, 8);
    let digit_to_chars: FxHashMap<_, FxHashSet<_>> = hashmap! {
        0 => btreeset!{b'a',b'b',b'c',b'e',b'f',b'g'}.into_iter().collect(),
        1 => btreeset!{b'c',b'f'}.into_iter().collect(),
        2 => btreeset!{b'a',b'c',b'd',b'e',b'g'}.into_iter().collect(),
        3 => btreeset!{b'a',b'c',b'd',b'f',b'g'}.into_iter().collect(),
        4 => btreeset!{b'b',b'c',b'd',b'f'}.into_iter().collect(),
        5 => btreeset!{b'a',b'b',b'd',b'f',b'g'}.into_iter().collect(),
        6 => btreeset!{b'a',b'b',b'd',b'e',b'f',b'g'}.into_iter().collect(),
        7 => btreeset!{b'a',b'c',b'f'}.into_iter().collect(),
        8 => btreeset!{b'a',b'b',b'c',b'd',b'e',b'f',b'g'}.into_iter().collect(),
        9 => btreeset!{b'a',b'b',b'c',b'd',b'f',b'g'}.into_iter().collect(),
    }
    .into_iter()
    .collect();
    let chars_to_digit: FxHashMap<V, u8> = digit_to_chars
        .iter()
        .map(|(d, c)| {
            let mut tmp: V = c.iter().cloned().collect();
            debug_assert!(!tmp.spilled());
            tmp.sort_unstable();
            (tmp, *d)
        })
        .collect();
    let mut chars_to_count: FxHashMap<u8, usize> = FxHashMap::default();
    for chars in digit_to_chars.values() {
        for c in chars {
            *chars_to_count.entry(*c).or_default() += 1;
        }
    }
    let mut count_to_chars: FxHashMap<usize, FxHashSet<u8>> = FxHashMap::default();
    for (c, count) in &chars_to_count {
        count_to_chars.entry(*count).or_default().insert(*c);
    }
    let mut cands: FxHashMap<_, FxHashSet<_>> = FxHashMap::default();
    cands.insert(
        b'a',
        btreeset! {b'a',b'b',b'c',b'd',b'e',b'f',b'g'}
            .into_iter()
            .collect(),
    );
    cands.insert(
        b'b',
        btreeset! {b'a',b'b',b'c',b'd',b'e',b'f',b'g'}
            .into_iter()
            .collect(),
    );
    cands.insert(
        b'c',
        btreeset! {b'a',b'b',b'c',b'd',b'e',b'f',b'g'}
            .into_iter()
            .collect(),
    );
    cands.insert(
        b'd',
        btreeset! {b'a',b'b',b'c',b'd',b'e',b'f',b'g'}
            .into_iter()
            .collect(),
    );
    cands.insert(
        b'e',
        btreeset! {b'a',b'b',b'c',b'd',b'e',b'f',b'g'}
            .into_iter()
            .collect(),
    );
    cands.insert(
        b'f',
        btreeset! {b'a',b'b',b'c',b'd',b'e',b'f',b'g'}
            .into_iter()
            .collect(),
    );
    cands.insert(
        b'g',
        btreeset! {b'a',b'b',b'c',b'd',b'e',b'f',b'g'}
            .into_iter()
            .collect(),
    );
    let mut c = 0;
    for line in lines {
        let mut line: Vec<V> = line.split(' ').map(|s| conv(s)).collect();
        line.remove(10);
        let pats = &line[0..10];
        let out = &line[10..];
        let mut cands = cands.clone();
        let mut counts_of_chars: FxHashMap<u8, usize> = FxHashMap::default();
        for pat in pats {
            for c in pat {
                *counts_of_chars.entry(*c).or_default() += 1;
            }
        }
        for (c, count) in &counts_of_chars {
            let candidate = &count_to_chars[count];
            if candidate.len() == 1 {
                let s: u8 = *candidate.iter().next().unwrap();
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

        let mut lookup = [0u8; 7];
        for (from, to) in &cands {
            lookup[(from - A) as usize] = *to.iter().next().unwrap();
        }

        c += out
            .iter()
            .fold(0, |acc, p| 10 * acc + trans(p, lookup, &chars_to_digit));
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
