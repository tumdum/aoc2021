use once_cell::sync::Lazy;
use regex::Regex;
use std::io::BufRead;
use std::time::{Duration, Instant};

type V<T> = smallvec::SmallVec<[T; 11]>;
type P = (i32, i32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Cube {
    x: P,
    y: P,
    z: P,
}

impl Cube {
    fn new(x: P, y: P, z: P) -> Self {
        Self {
            x: (x.0, x.1 + 1),
            y: (y.0, y.1 + 1),
            z: (z.0, z.1 + 1),
        }
    }

    fn size(&self) -> usize {
        let x = (self.x.1 - self.x.0) as usize;
        let y = (self.y.1 - self.y.0) as usize;
        let z = (self.z.1 - self.z.0) as usize;
        let ret = (x * y * z) as usize;
        ret
    }

    fn intersect(&self, other: &Self) -> bool {
        !self.is_outside(other)
    }

    fn is_outside(&self, other: &Self) -> bool {
        self.x.1 <= other.x.0
            || self.x.0 >= other.x.1
            || self.y.1 <= other.y.0
            || self.y.0 >= other.y.1
            || self.z.1 <= other.z.0
            || self.z.0 >= other.z.1
    }

    fn subtract(&self, other: &Self) -> Option<V<Self>> {
        if !self.intersect(other) {
            return None;
        }
        let mut xs = vec![self.x.0, self.x.1, other.x.0, other.x.1];
        let mut ys = vec![self.y.0, self.y.1, other.y.0, other.y.1];
        let mut zs = vec![self.z.0, self.z.1, other.z.0, other.z.1];

        xs.sort();
        xs.dedup();
        ys.sort();
        ys.dedup();
        zs.sort();
        zs.dedup();

        let mut ret = V::new();

        for x in xs.windows(2) {
            for y in ys.windows(2) {
                for z in zs.windows(2) {
                    let c = Cube {
                        x: (x[0], x[1]),
                        y: (y[0], y[1]),
                        z: (z[0], z[1]),
                    };
                    if c.size() > 0 {
                        let in_a = self.intersect(&c);
                        let in_b = other.intersect(&c);
                        if in_a && !in_b {
                            ret.push(c);
                        }
                    }
                }
            }
        }

        Some(ret)
    }
}

#[derive(Clone, Debug)]
struct Shape {
    cubes: Vec<Cube>,
}

impl Shape {
    fn subtract(self, other: &Self) -> Self {
        let mut cubes = self.cubes;

        for other_cube in &other.cubes {
            let mut new_ones = V::new();
            cubes.retain(|c| {
                if let Some(cand) = c.subtract(&other_cube) {
                    new_ones.extend(cand);
                    false
                } else {
                    true
                }
            });
            cubes.extend(new_ones);
        }

        Self { cubes }
    }

    fn add(mut self, other: &Self) -> Self {
        let tmp = other.clone().subtract(&self);
        self.cubes.extend(tmp.cubes);
        self
    }
}

static RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"(.*) x=(.+)\.\.(.+),y=(.+)\.\.(.+),z=(.+)\.\.(.+)"#).unwrap());

fn parse(s: &str) -> (bool, P, P, P) {
    let c = RE.captures(s).unwrap();
    let b = &c[1] == "on";
    let x_min = c[2].parse().unwrap();
    let x_max = c[3].parse().unwrap();
    let y_min = c[4].parse().unwrap();
    let y_max = c[5].parse().unwrap();
    let z_min = c[6].parse().unwrap();
    let z_max = c[7].parse().unwrap();
    (b, (x_min, x_max), (y_min, y_max), (z_min, z_max))
}

pub fn solve(input: &mut dyn BufRead, verify_expected: bool, output: bool) -> Duration {
    let input: Vec<(bool, P, P, P)> = input.lines().map(|s| parse(&s.unwrap())).collect();

    let s = Instant::now();

    let mut shape = Shape { cubes: vec![] };
    for (b, (mut x_min, mut x_max), (mut y_min, mut y_max), (mut z_min, mut z_max)) in input.clone()
    {
        if x_min < -50 {
            x_min = -50;
        }
        if y_min < -50 {
            y_min = -50;
        }
        if z_min < -50 {
            z_min = -50;
        }
        if x_max > 50 {
            x_max = 50;
        }
        if y_max > 50 {
            y_max = 50;
        }
        if z_max > 50 {
            z_max = 50;
        }
        if x_min > x_max || y_min > y_max || z_min > z_max {
            continue;
        }
        let cube = Cube::new((x_min, x_max), (y_min, y_max), (z_min, z_max));
        if b {
            shape = shape.add(&Shape { cubes: vec![cube] });
        } else {
            shape = shape.subtract(&Shape { cubes: vec![cube] });
        }
    }

    let part1 = shape.cubes.iter().map(|c| c.size()).sum::<usize>();

    let operations: Vec<(bool, Cube)> = input
        .iter()
        .cloned()
        .map(|(b, x, y, z)| (b, Cube::new(x, y, z)))
        .collect();

    let mut shape = Shape { cubes: vec![] };

    for (add, cube) in &operations {
        if *add {
            shape = shape.add(&Shape { cubes: vec![*cube] });
        } else {
            shape = shape.subtract(&Shape { cubes: vec![*cube] });
        }
    }

    let part2 = shape.cubes.iter().map(|c| c.size()).sum::<usize>();

    let e = s.elapsed();
    if verify_expected {
        assert_eq!(553201, part1);
        assert_eq!(1263946820845866, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    e
}
