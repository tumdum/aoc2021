use smallvec::SmallVec;
use std::io::BufRead;
use std::time::{Duration, Instant};

pub fn solve(input: &mut dyn BufRead, verify_expected: bool, output: bool) -> Duration {
    let nums: Vec<SmallVec<[u8; 12]>> = input
        .lines()
        .map(|s| {
            s.unwrap()
                .chars()
                .map(|c| c.to_digit(2).unwrap() as u8)
                .collect()
        })
        .collect();

    let s = Instant::now();
    let part1 = part_one(&nums);
    let part2 = part_two(&nums);
    let e = s.elapsed();
    if verify_expected {
        assert_eq!(2954600, part1);
        assert_eq!(1662846, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    e
}

fn most_frequent_bit(n: usize, nums: &[SmallVec<[u8; 12]>]) -> u8 {
    let mut f = [0usize; 2];
    nums.iter().for_each(|num| f[num[n] as usize] += 1);
    (f[0] > f[1]) as u8
}

fn part_one(nums: &[SmallVec<[u8; 12]>]) -> i64 {
    let len = nums[0].len();
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for i in 0..len {
        if most_frequent_bit(i, nums) == 0 {
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push('1');
            epsilon.push('0');
        }
    }
    let gamma = i64::from_str_radix(&gamma, 2).unwrap();
    let epsilon = i64::from_str_radix(&epsilon, 2).unwrap();
    gamma * epsilon
}

fn part_two(all: &[SmallVec<[u8; 12]>]) -> i64 {
    let mut most = all.to_vec();
    let mut least = all.to_vec();
    for i in 0..all[0].len() {
        if most.len() > 1 {
            let b = most_frequent_bit(i, &most);
            most.retain(|n| n[i] == b);
        }
        if least.len() > 1 {
            let b = most_frequent_bit(i, &least);
            least.retain(|n| n[i] != b);
        }
    }
    let most: String = most[0]
        .iter()
        .map(|b| if *b == 0 { '0' } else { '1' })
        .collect();
    let least: String = least[0]
        .iter()
        .map(|b| if *b == 0 { '0' } else { '1' })
        .collect();
    let most = i64::from_str_radix(&most, 2).unwrap();
    let least = i64::from_str_radix(&least, 2).unwrap();
    most * least
}
