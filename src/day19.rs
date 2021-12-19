use rustc_hash::{FxHashMap, FxHashSet};
use std::convert::Infallible;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::io::BufRead;
use std::ops::{Add, Sub};
use std::str::FromStr;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
struct P3 {
    x: i64,
    y: i64,
    z: i64,
}

impl P3 {
    fn rot_x(&self) -> Self {
        Self {
            x: self.x,
            y: -self.z,
            z: self.y,
        }
    }

    fn rot_y(&self) -> Self {
        Self {
            x: -self.z,
            y: self.y,
            z: self.x,
        }
    }

    fn rot_z(&self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
            z: self.z,
        }
    }

    fn dist(&self) -> usize {
        (self.x.abs() + self.y.abs() + self.z.abs()) as usize
    }
}

fn rot_x(p: &P3, n: usize) -> P3 {
    let mut ret = *p;
    for _ in 0..n {
        ret = ret.rot_x();
    }
    ret
}

fn rot_y(p: &P3, n: usize) -> P3 {
    let mut ret = *p;
    for _ in 0..n {
        ret = ret.rot_y();
    }
    ret
}

fn rot_z(p: &P3, n: usize) -> P3 {
    let mut ret = *p;
    for _ in 0..n {
        ret = ret.rot_z();
    }
    ret
}

impl Debug for P3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl Sub for P3 {
    type Output = P3;

    fn sub(self, other: P3) -> P3 {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Add for P3 {
    type Output = P3;

    fn add(self, other: P3) -> P3 {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl FromStr for P3 {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<P3, Infallible> {
        let mut s = s.split(',');
        let x = s.next().unwrap().parse().unwrap();
        let y = s.next().unwrap().parse().unwrap();
        let z = s.next().unwrap().parse().unwrap();
        Ok(P3 { x, y, z })
    }
}

fn parse_scanner(s: &[String]) -> (usize, Vec<P3>) {
    let id = s[0].split(' ').nth(2).unwrap().parse();
    let pos = s[1..].iter().map(|s| s.parse().unwrap()).collect();
    (id.unwrap(), pos)
}

fn relative_to_nth(points: &[P3], n: usize) -> (P3, FxHashSet<P3>) {
    let f = |p: &P3| *p - points[n];
    (f(&P3 { x: 0, y: 0, z: 0 }), points.iter().map(f).collect())
}

fn generate_all_cands(points: &[P3]) -> Vec<(P3, FxHashSet<P3>)> {
    let mut ret = vec![];
    for i in 0..points.len() {
        ret.push(relative_to_nth(points, i));
    }
    let mut real_ret = vec![];
    for x_rot in [0, 1, 2, 3] {
        for y_rot in [0, 1, 2, 3] {
            for z_rot in [0, 1, 2, 3] {
                let f = |p: &P3| rot_z(&rot_y(&rot_x(p, x_rot), y_rot), z_rot);
                for (origin, points) in &ret {
                    let new_origin = f(origin);
                    let new_points = points.iter().map(|p| f(p)).collect();
                    real_ret.push((new_origin, new_points));
                }
            }
        }
    }

    real_ret
}

fn overlap<'a, 'b>(
    (base_origin, base_points): &'a (P3, FxHashSet<P3>),
    candidates: &'b [(P3, FxHashSet<P3>)],
) -> Option<(P3, &'b FxHashSet<P3>)> {
    for (origin, cand) in candidates {
        let common = cand.intersection(&base_points).count();
        if common >= 12 {
            let origin_in_zero = *origin - *base_origin;
            return Some((origin_in_zero, cand));
        }
    }
    None
}

fn find_overlap_with(
    base: &[P3],
    cache: &FxHashMap<usize, Vec<(P3, FxHashSet<P3>)>>,
) -> Option<(usize, P3, P3, FxHashSet<P3>)> {
    for i in 0..base.len() {
        let base = relative_to_nth(base, i);
        for (id, candidates) in cache {
            if let Some((origin, points)) = overlap(&base, candidates) {
                let all: FxHashSet<P3> = base
                    .1
                    .iter()
                    .cloned()
                    .chain(points.iter().cloned())
                    .collect();
                return Some((*id, origin, base.0, all));
            }
        }
    }
    None
}

pub fn solve(input: &mut dyn BufRead, verify_expected: bool, output: bool) -> Duration {
    let input: Vec<String> = input.lines().map(|s| s.unwrap()).collect();
    let mut scanners: FxHashMap<_, _> = input.split(|s| s.is_empty()).map(parse_scanner).collect();

    let mut cache: FxHashMap<_, _> = scanners
        .iter()
        .map(|(id, points)| (*id, generate_all_cands(points)))
        .collect();

    let s = Instant::now();
    let mut last_origin = P3::default();

    let mut origins = vec![last_origin];
    let mut zero = scanners[&0].clone();
    cache.remove(&0);
    while cache.len() > 0 {
        let (id, origin, new_base_origin, all_points_in_zero) =
            find_overlap_with(&zero, &cache).unwrap();
        origins.push(origin - last_origin);
        last_origin = last_origin + new_base_origin;
        zero = all_points_in_zero.into_iter().collect();
        cache.remove(&id);
    }
    let part1 = zero.len();
    // zero.iter_mut().for_each(|p| *p = *p - last_origin);

    let mut part2 = 0;
    for (i, a) in origins.iter().enumerate() {
        for (j, b) in origins.iter().enumerate() {
            if i != j {
                let d = (*a - *b).dist();
                if d > part2 {
                    part2 = d;
                }
            }
        }
    }

    let e = s.elapsed();
    if verify_expected {
        assert_eq!(342, part1);
        assert_eq!(9668, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    e
}
