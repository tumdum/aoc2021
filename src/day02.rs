use std::io::BufRead;
use std::str::FromStr;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up(i64),
    Down(i64),
    Forward(i64),
}

impl FromStr for Dir {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let mut s = s.split(' ');
        let kind = s.next().unwrap();
        let v = s.next().unwrap().parse().unwrap();
        match kind {
            "up" => Ok(Dir::Up(v)),
            "down" => Ok(Dir::Down(v)),
            "forward" => Ok(Dir::Forward(v)),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Pos {
    h: i64,
    d: i64,
    aim: i64,
}

impl Pos {
    fn apply_move(self, dir: Dir) -> Pos {
        match dir {
            Dir::Up(v) => Pos {
                d: self.d - v,
                ..self
            },
            Dir::Down(v) => Pos {
                d: self.d + v,
                ..self
            },
            Dir::Forward(v) => Pos {
                h: self.h + v,
                ..self
            },
        }
    }
    fn apply_move2(self, dir: Dir) -> Pos {
        match dir {
            Dir::Up(v) => Pos {
                aim: self.aim - v,
                ..self
            },
            Dir::Down(v) => Pos {
                aim: self.aim + v,
                ..self
            },
            Dir::Forward(v) => Pos {
                h: self.h + v,
                d: self.d + self.aim * v,
                ..self
            },
        }
    }
}

pub fn solve(input: &mut dyn BufRead, verify_expected: bool) -> Duration {
    let moves: Vec<Dir> = input.lines().map(|s| s.unwrap().parse().unwrap()).collect();

    let s = Instant::now();
    let pos = moves.iter().fold(Pos::default(), |p, m| p.apply_move(*m));
    let part1 = pos.h * pos.d;
    let pos = moves.iter().fold(Pos::default(), |p, m| p.apply_move2(*m));
    let part2 = pos.h * pos.d;
    let e = s.elapsed();
    if verify_expected {
        assert_eq!(1698735, part1);
        assert_eq!(1594785890, part2);
    }
    println!("\t{}", part1);
    println!("\t{}", part2);
    e
}
