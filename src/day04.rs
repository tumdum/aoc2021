use std::cell::RefCell;
use std::collections::HashMap;
use std::io::BufRead;
use std::time::{Duration, Instant};

const LEN: u8 = 5;

struct Board {
    inv: HashMap<u8, (u8, u8)>,
    sum: u32,
    cols: [u8; LEN as usize],
    rows: [u8; LEN as usize],
}

impl Board {
    fn new(v: Vec<Vec<u32>>) -> Self {
        let mut inv = HashMap::new();
        let mut sum = 0;
        for i in 0..v.len() {
            for j in 0..v[i].len() {
                inv.insert(v[i][j] as u8, (i as u8, j as u8));
                sum += v[i][j];
            }
        }
        Self {
            inv,
            sum,
            cols: [0; LEN as usize],
            rows: [0; LEN as usize],
        }
    }

    fn mark(&mut self, v: u32) -> Option<u32> {
        if let Some((i, j)) = self.inv.get(&(v as u8)) {
            self.sum -= v;
            if self.cols[*j as usize] == LEN - 1 {
                return Some(self.sum);
            }
            self.cols[*j as usize] += 1;
            if self.rows[*i as usize] == LEN - 1 {
                return Some(self.sum);
            }
            self.rows[*i as usize] += 1;
        }
        None
    }
}

fn parse_board(b: Vec<String>) -> RefCell<Board> {
    Board::new(
        b.into_iter()
            .map(|r| r.split(' ').flat_map(|v| v.parse().ok()).collect())
            .collect(),
    )
    .into()
}

pub fn solve(input: &mut dyn BufRead, verify_expected: bool, output: bool) -> Duration {
    let input: Vec<_> = input.lines().map(|s| s.unwrap()).collect();
    let nums: Vec<u32> = input[0].split(',').map(|v| v.parse().unwrap()).collect();
    let mut boards: Vec<_> = input[2..]
        .split(|s| s.is_empty())
        .map(|v| v.to_vec())
        .map(parse_board)
        .collect();
    let s = Instant::now();
    let mut wins = vec![];
    nums.into_iter().for_each(|n| {
        boards.retain(|b| {
            if let Some(sum) = b.borrow_mut().mark(n) {
                wins.push(sum * n);
                false
            } else {
                true
            }
        })
    });
    let part1 = *wins.first().unwrap();
    let part2 = *wins.last().unwrap();
    let elapsed = s.elapsed();
    if verify_expected {
        assert_eq!(49686, part1);
        assert_eq!(26878, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    elapsed
}
