use cached::proc_macro::cached;
use std::io::BufRead;
use std::time::{Duration, Instant};

fn roll(dice: &mut u64, rolls: &mut u64) -> u64 {
    *rolls += 1;
    let v = *dice;
    let mut d = *dice + 1;
    if d == 101 {
        d = 1;
    }
    *dice = d;
    v
}
fn part1(mut pos: [u64; 2]) -> u64 {
    let mut rolls = 0;
    let mut scores = [0, 0];
    let mut dice = 1;

    loop {
        let a = roll(&mut dice, &mut rolls);
        let b = roll(&mut dice, &mut rolls);
        let c = roll(&mut dice, &mut rolls);
        pos[0] += (a + b + c) % 10;
        if pos[0] > 10 {
            pos[0] -= 10;
        }
        debug_assert!(pos[0] >= 1 && pos[0] <= 10);
        scores[0] += pos[0];
        if scores[0] >= 1000 {
            break;
        }

        let a = roll(&mut dice, &mut rolls);
        let b = roll(&mut dice, &mut rolls);
        let c = roll(&mut dice, &mut rolls);
        pos[1] += (a + b + c) % 10;
        if pos[1] > 10 {
            pos[1] -= 10;
        }
        debug_assert!(pos[1] >= 1 && pos[1] <= 10);
        scores[1] += pos[1];
        if scores[1] >= 1000 {
            break;
        }
    }

    rolls * *scores.iter().min().unwrap()
}

#[cached]
fn part2(pos: [u64; 2], scores: [u64; 2], player: usize) -> [u64; 2] {
    const SCORES: [u64; 27] = [
        3, 4, 4, 4, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 6, 7, 7, 7, 7, 7, 7, 8, 8, 8, 9,
    ];
    let mut wins = [0; 2];
    for s in SCORES {
        let mut pos = pos;
        let mut scores = scores;
        pos[player] += s;
        if pos[player] > 10 {
            pos[player] -= 10;
        }
        scores[player] += pos[player];
        if scores[player] >= 21 {
            wins[player] += 1;
        } else {
            let sub_wins = part2(pos, scores, if player == 0 { 1 } else { 0 });
            wins[0] += sub_wins[0];
            wins[1] += sub_wins[1];
        }
    }
    wins
}

fn parse(s: &str) -> u64 {
    s.split(' ').last().unwrap().parse().unwrap()
}

pub fn solve(input: &mut dyn BufRead, verify_expected: bool, output: bool) -> Duration {
    let input: Vec<u64> = input.lines().map(|s| parse(&s.unwrap())).collect();
    let input: [u64; 2] = input.try_into().unwrap();

    let s = Instant::now();

    let part1 = part1(input);
    let part2 = *part2(input, [0, 0], 0).iter().max().unwrap();

    let e = s.elapsed();
    if verify_expected {
        assert_eq!(556206, part1);
        assert_eq!(630797200227453, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    e
}
