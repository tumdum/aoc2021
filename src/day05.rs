use std::error::Error;
use std::io::BufRead;
use std::ops::{AddAssign, Sub};
use std::str::FromStr;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn normalize(&self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }

    fn len(&self) -> i64 {
        if self.x != 0 {
            self.x.abs()
        } else {
            self.y.abs()
        }
    }
}

impl FromStr for Point {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Box<dyn Error>> {
        let mut s = s.split(',');
        let x = s.next().ok_or("x")?.parse()?;
        let y = s.next().ok_or("y")?.parse()?;
        Ok(Self { x, y })
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl Sub<Point> for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_straight(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }
}

impl FromStr for Line {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Box<dyn Error>> {
        let mut s = s.split(" -> ");
        let start = s.next().ok_or("start")?.parse()?;
        let end = s.next().ok_or("end")?.parse()?;
        Ok(Self { start, end })
    }
}

fn count_overlaps(lines: Vec<Line>, mut count: usize, w: i64, map: &mut [u8]) -> usize {
    for l in lines {
        let dif = l.end - l.start;
        let len = dif.len() + 1;
        let dif = dif.normalize();
        let mut cur = l.start;
        for _ in 0..len {
            let v = &mut map[(cur.y * w + cur.x) as usize];
            match v {
                0 => *v = 1,
                1 => {
                    *v = 2;
                    count += 1;
                }
                _ => {}
            }
            cur += dif;
        }
    }

    count
}

pub fn solve(input: &mut dyn BufRead, verify_expected: bool, output: bool) -> Duration {
    let mut lines: Vec<Line> = input.lines().map(|s| s.unwrap().parse().unwrap()).collect();
    let s = Instant::now();
    let max_x = (lines
        .iter()
        .flat_map(|l| [l.start, l.end])
        .map(|p| p.x)
        .max()
        .unwrap()
        + 1) as usize;
    let max_y = (lines
        .iter()
        .flat_map(|l| [l.start, l.end])
        .map(|p| p.y)
        .max()
        .unwrap()
        + 1) as usize;

    let mut diagonals = vec![];
    lines.retain(|l| {
        if l.is_straight() {
            true
        } else {
            diagonals.push(*l);
            false
        }
    });
    let straight = lines;

    let mut map = vec![0; (max_x * max_y) as usize];

    let part1 = count_overlaps(straight, 0, max_x as i64, &mut map);
    let part2 = count_overlaps(diagonals, part1, max_x as i64, &mut map);
    let e = s.elapsed();
    if verify_expected {
        assert_eq!(5632, part1);
        assert_eq!(22213, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    e
}
