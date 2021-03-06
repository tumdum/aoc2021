use itertools::iproduct;
use rustc_hash::{FxHashMap, FxHashSet};
use std::convert::Infallible;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::io::BufRead;
use std::ops::{Add, Sub};
use std::str::FromStr;
use std::time::{Duration, Instant};

type V<T> = smallvec::SmallVec<[T; 26]>;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
struct P3 {
    x: i32,
    y: i32,
    z: i32,
}

// Seems that the fxhash is slower than this:
impl Hash for P3 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32(self.x ^ self.y ^ self.z);
    }
}

impl P3 {
    fn rot_x(self) -> Self {
        Self {
            x: self.x,
            y: -self.z,
            z: self.y,
        }
    }

    const fn rot_y(self) -> Self {
        Self {
            x: -self.z,
            y: self.y,
            z: self.x,
        }
    }

    const fn rot_z(self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
            z: self.z,
        }
    }

    const fn dist(&self) -> usize {
        (self.x.abs() + self.y.abs() + self.z.abs()) as usize
    }
}

fn rot_x(mut p: P3, n: usize) -> P3 {
    for _ in 0..n {
        p = p.rot_x();
    }
    p
}

fn rot_y(mut p: P3, n: usize) -> P3 {
    for _ in 0..n {
        p = p.rot_y();
    }
    p
}

fn rot_z(mut p: P3, n: usize) -> P3 {
    for _ in 0..n {
        p = p.rot_z();
    }
    p
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

fn generate_all_cands(points: &[P3]) -> Vec<(P3, V<P3>)> {
    let ret: Vec<_> = (0..points.len())
        .map(|i| relative_to_nth(points, i))
        .collect();
    let mut real_ret: FxHashMap<P3, V<P3>> = FxHashMap::default();
    for x_rot in [0, 1] {
        // this reduced number of x rotations still produces all required rotations
        for y_rot in [0, 1, 2, 3] {
            for z_rot in [0, 1, 2, 3] {
                let f = |p: P3| rot_z(rot_y(rot_x(p, x_rot), y_rot), z_rot);
                for (origin, points) in &ret {
                    real_ret
                        .entry(f(*origin))
                        .or_insert(points.into_iter().map(|p| f(*p)).collect());
                }
            }
        }
    }

    real_ret
        .into_iter()
        .map(|(orig, points)| (orig, points))
        .collect()
}

fn overlap<'a, 'b>(
    (base_origin, base_points): &'a (P3, FxHashSet<P3>),
    candidates: &'b [(P3, V<P3>)],
) -> Option<(P3, &'b [P3])> {
    for (origin, cand) in candidates {
        let mut cand_size = cand.len();
        for c in cand {
            if !base_points.contains(c) {
                cand_size -= 1;
                if cand_size < 12 {
                    break;
                }
            }
        }
        if cand_size < 12 {
            continue;
        }
        let origin_in_zero = *origin - *base_origin;
        return Some((origin_in_zero, cand));
    }
    None
}

fn find_overlap_with(
    base: &[P3],
    cache: &[Vec<(P3, V<P3>)>],
) -> Option<(usize, P3, P3, FxHashSet<P3>)> {
    for i in 0..base.len() {
        let mut base = relative_to_nth(base, i);
        for (id, candidates) in cache.iter().enumerate() {
            if let Some((origin, points)) = overlap(&base, candidates) {
                base.1.extend(points);
                return Some((id, origin, base.0, base.1));
            }
        }
    }
    None
}

pub fn solve(input: &mut dyn BufRead, verify_expected: bool, output: bool) -> Duration {
    let input: Vec<String> = input.lines().map(|s| s.unwrap()).collect();
    let scanners: FxHashMap<_, _> = input.split(|s| s.is_empty()).map(parse_scanner).collect();

    let s = Instant::now();
    let mut cache: Vec<_> = scanners
        .iter()
        .map(|(_, points)| generate_all_cands(points))
        .collect();

    let mut last_origin = P3::default();

    let mut origins = vec![last_origin];
    let mut zero = scanners[&0].clone();
    cache.remove(0);
    while cache.len() > 0 {
        let (index, origin, new_base_origin, all_points_in_zero) =
            find_overlap_with(&zero, &cache).unwrap();
        origins.push(origin - last_origin);
        last_origin = last_origin + new_base_origin;
        zero = all_points_in_zero.into_iter().collect();
        cache.remove(index);
    }
    let part1 = zero.len();

    let part2 = iproduct!(&origins, &origins)
        .filter(|(a, b)| a != b)
        .map(|(a, b)| (*a - *b).dist())
        .max()
        .unwrap();

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
