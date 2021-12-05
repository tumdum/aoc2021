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
struct Line {
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

fn count_overlaps(
    lines: Vec<Line>,
    start_count: usize,
    map: &mut [Vec<u8>],
    is_straight: bool,
) -> usize {
    let lines: Vec<_> = lines
        .iter()
        .filter(|l| is_straight == l.is_straight())
        .cloned()
        .collect();

    let mut count = start_count;

    for l in lines {
        let dif = l.end - l.start;
        let len = dif.len() + 1;
        let dif = dif.normalize();
        let mut cur = l.start;
        for _ in 0..len {
            let v = &mut map[cur.x as usize][cur.y as usize];
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
pub fn solve(input: &mut dyn BufRead, verify_expected: bool) -> Duration {
    let lines: Vec<Line> = input.lines().map(|s| s.unwrap().parse().unwrap()).collect();
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

    let row = vec![0u8; max_y as usize];
    let mut map = vec![row; max_x as usize];

    let part1 = count_overlaps(lines.clone(), 0, &mut map, true);
    let part2 = count_overlaps(lines, part1, &mut map, false);
    let e = s.elapsed();
    if verify_expected {
        assert_eq!(5632, part1);
        assert_eq!(22213, part2);
    }
    println!("\t{}", part1);
    println!("\t{}", part2);
    e
}
