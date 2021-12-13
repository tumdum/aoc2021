use rustc_hash::FxHashSet;
use std::io::BufRead;
use std::time::{Duration, Instant};

#[derive(Clone)]
struct Paper {
    pos: FxHashSet<(i32, i32)>,
    max: (i32, i32),
}

impl Paper {
    fn new(m: FxHashSet<(i32, i32)>) -> Self {
        let max_x = m.iter().map(|p| p.0).max().unwrap();
        let max_y = m.iter().map(|p| p.1).max().unwrap();

        Self {
            pos: m,
            max: (max_x, max_y),
        }
    }

    fn print(&self) {
        println!();
        let (max_x, max_y) = self.max;
        for r in 0..=max_y {
            print!("{:>2} ", r);
            for c in 0..=max_x {
                if self.pos.contains(&(c, r)) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!()
        }
    }

    fn fold_y(&self, y: i32) -> Self {
        let pos = self
            .pos
            .iter()
            .map(|p| if p.1 < y { *p } else { (p.0, self.max.1 - p.1) })
            .collect();
        Self {
            pos,
            max: (self.max.0, y - 1),
        }
    }

    fn fold_x(&self, x: i32) -> Self {
        let pos = self
            .pos
            .iter()
            .map(|p| if p.0 < x { *p } else { (self.max.0 - p.0, p.1) })
            .collect();
        Self {
            pos,
            max: (x - 1, self.max.1),
        }
    }

    fn dots(&self) -> usize {
        self.pos.len()
    }
}

pub fn solve(input: &mut dyn BufRead, verify_expected: bool, output: bool) -> Duration {
    let input: Vec<String> = input.lines().map(|s| s.unwrap()).collect();

    let s = Instant::now();

    let mut input = input.split(|s| s.is_empty());
    let pos: FxHashSet<(i32, i32)> = input
        .next()
        .unwrap()
        .iter()
        .map(|s| {
            let mut s = s.split(',');
            let x = s.next().unwrap().parse().unwrap();
            let y = s.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect();
    let folds = input.next().unwrap();

    let mut paper = Paper::new(pos);
    let mut once = true;
    let mut part1 = 0;

    for fold in folds {
        let mut fold = fold.split(' ');
        fold.next();
        fold.next();
        let mut fold = fold.next().unwrap().split('=');
        let axis = fold.next().unwrap();
        let val: i32 = fold.next().unwrap().parse().unwrap();

        paper = if axis == "x" {
            paper.fold_x(val)
        } else {
            paper.fold_y(val)
        };

        if once {
            part1 = paper.dots();
            once = false;
        }
    }

    let e = s.elapsed();
    if verify_expected {
        assert_eq!(753, part1);
        assert_eq!(98, paper.dots());
    }
    if output {
        println!("\t{}", part1);
        paper.print();
    }
    e
}
