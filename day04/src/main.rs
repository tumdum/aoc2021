use std::collections::HashMap;
use std::io::{stdin, BufRead};
use std::time::Instant;

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
            self.cols[*j as usize] += 1;
            self.rows[*i as usize] += 1;
            if self.cols[*j as usize] == LEN || self.rows[*i as usize] == LEN {
                return Some(self.sum);
            }
        }
        None
    }
}

fn parse_board(b: Vec<String>) -> Board {
    Board::new(
        b.into_iter()
            .map(|r| r.split(' ').flat_map(|v| v.parse().ok()).collect())
            .collect(),
    )
}

fn main() {
    let input: Vec<String> = stdin().lock().lines().map(|s| s.unwrap()).collect();
    let nums: Vec<u32> = input[0].split(',').map(|v| v.parse().unwrap()).collect();
    let mut boards: Vec<_> = input[2..]
        .split(|s| s.is_empty())
        .map(|v| v.to_vec())
        .map(parse_board)
        .collect();
    let s = Instant::now();
    let mut wins = vec![];
    for n in nums {
        let mut new_board: Vec<Board> = vec![];
        for mut b in boards {
            if let Some(sum) = b.mark(n) {
                wins.push(sum * n)
            } else {
                new_board.push(b);
            }
        }
        if new_board.is_empty() {
            break;
        }
        boards = new_board;
    }
    let elapsed = s.elapsed();
    println!("{:?}", wins.first());
    println!("{:?}", wins.last());
    println!("elapsed {:?}", elapsed);
}
