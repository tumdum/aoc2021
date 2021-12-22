use regex::Regex;
use rustc_hash::FxHashSet;
use std::collections::HashSet;
use std::io::BufRead;
use std::time::{Duration, Instant};

type P = (i32, i32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Cube {
    x: P,
    y: P,
    z: P,
}

impl Cube {
    fn size(&self) -> usize {
        let x = (self.x.1 - self.x.0) as usize;
        let y = (self.y.1 - self.y.0) as usize;
        let z = (self.z.1 - self.z.0) as usize;
        (x * y * z) as usize
    }
}

fn intersect(a: P, b: P) -> Vec<P> {
    let mut v = intersect_impl(a, b);
    v.retain(|p| p.0 <= p.1);
    v
}

fn intersect_impl((a_min, a_max): P, (b_min, b_max): P) -> Vec<P> {
    if a_min <= b_min && b_min <= a_max {
        if b_max <= a_max {
            vec![(a_min, b_min - 1), (b_min, b_max), (b_max + 1, a_max)]
        } else {
            vec![(a_min, b_min - 1), (b_min, a_max), (a_max + 1, b_max)]
        }
    } else if a_min <= b_max && b_max <= a_max {
        vec![(b_min, a_min - 1), (a_min, b_max), (b_max + 1, a_max)]
    } else if b_min <= a_min && a_max <= b_max {
        vec![(b_min, a_min - 1), (a_min, a_max), (a_max + 1, b_max)]
    } else {
        vec![]
    }
}

fn parse(s: &str) -> (bool, P, P, P) {
    let re = Regex::new(r#"(.*) x=(.+)\.\.(.+),y=(.+)\.\.(.+),z=(.+)\.\.(.+)"#).unwrap();
    let c = re.captures(s).unwrap();
    let b = &c[1] == "on";
    let x_min = c[2].parse().unwrap();
    let x_max = c[3].parse().unwrap();
    let y_min = c[4].parse().unwrap();
    let y_max = c[5].parse().unwrap();
    let z_min = c[6].parse().unwrap();
    let z_max = c[7].parse().unwrap();
    (b, (x_min, x_max), (y_min, y_max), (z_min, z_max))
}

fn find(ranges: &[i32], range: P) -> Vec<P> {
    // println!();
    // println!("{:?} vs {:?}", range, ranges);
    let mut v: Vec<i32> = ranges
        .iter()
        .filter(|p| range.0 <= **p && **p <= range.1)
        .cloned()
        .collect();
    // assert!(ranges.contains(&(*v.last().unwrap()+1)));
    v.push(*v.last().unwrap() + 1);
    // dbg!(&v);
    let ret: Vec<_> = v.windows(2).map(|v| (v[0], v[1])).collect();
    // dbg!(&ret);
    /*
    assert_eq!(range.0, ret.first().unwrap().0);
    assert_eq!(range.1+1, ret.last().unwrap().1);
    let expected : Vec<_> = (range.0..=range.1).collect();
    let mut got = vec![];
    for (a,b) in &ret {
        for v in *a..*b {
            got.push(v);
        }
    }
    assert_eq!(expected, got);
    */
    ret
}

pub fn solve(input: &mut dyn BufRead, verify_expected: bool, output: bool) -> Duration {
    let input: Vec<(bool, P, P, P)> = input.lines().map(|s| parse(&s.unwrap())).collect();

    let s = Instant::now();

    let mut points: HashSet<(i32, i32, i32)> = HashSet::new();

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
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                for z in z_min..=z_max {
                    if b {
                        points.insert((x, y, z));
                    } else {
                        points.remove(&(x, y, z));
                    }
                }
            }
        }
    }

    dbg!(points.len());

    let all_x: HashSet<_> = input
        .iter()
        .flat_map(|(_, x, _, _)| vec![x.0, x.1, x.1 + 1])
        .collect();
    let mut all_x: Vec<_> = all_x.into_iter().collect();
    all_x.sort();
    dbg!(all_x.len());

    let all_y: HashSet<_> = input
        .iter()
        .flat_map(|(_, _, y, _)| vec![y.0, y.1, y.1 + 1])
        .collect();
    let mut all_y: Vec<_> = all_y.into_iter().collect();
    all_y.sort();
    dbg!(all_y.len());

    let all_z: HashSet<_> = input
        .iter()
        .flat_map(|(_, _, _, z)| vec![z.0, z.1, z.1 + 1])
        .collect();
    let mut all_z: Vec<_> = all_z.into_iter().collect();
    all_z.sort();
    dbg!(all_z.len());
    dbg!(all_x.len() * all_y.len() * all_z.len());

    let mut cubes = FxHashSet::default();
    cubes.reserve(500000000);

    let l = input.len();

    for (i, (b, x, y, z)) in input.into_iter().enumerate() {
        println!("{:?}", (b, x, y, z));
        let xs = find(&all_x, x);
        let ys = find(&all_y, y);
        let zs = find(&all_z, z);
        println!("{}/{}: cubes: {}, {} x {} x {}", i, l, cubes.len(), xs.len(), ys.len(), zs.len());
        for x in &xs {
            for y in &ys {
                for z in &zs {
                    let cube = Cube {
                        x: *x,
                        y: *y,
                        z: *z,
                    };
                    if b {
                        cubes.insert(cube);
                    } else {
                        cubes.remove(&cube);
                    }
                }
            }
        }
    }

    dbg!(cubes.len());
    // println!("{:?}", &cubes);
    dbg!(cubes.iter().map(|c| c.size()).sum::<usize>());

    let e = s.elapsed();
    if verify_expected {
        // assert_eq!(1602, part1);
        // assert_eq!(1633, part2);
    }
    if output {
        // println!("\t{}", part1);
        // println!("\t{}", part2);
    }
    e
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interset() {
        assert_eq!(Vec::<P>::new(), intersect((0, 10), (11, 12)));
        assert_eq!(Vec::<P>::new(), intersect((13, 14), (11, 12)));
        assert_eq!(
            vec![(0, 9), (10, 10), (11, 12)],
            intersect((0, 10), (10, 12))
        );
        assert_eq!(vec![(0, 9), (10, 10)], intersect((0, 10), (10, 10)));
        assert_eq!(vec![(0, 8), (9, 10)], intersect((0, 10), (9, 10)));
        assert_eq!(vec![(0, 4), (5, 7), (8, 10)], intersect((0, 10), (5, 7)));
        assert_eq!(vec![(0, 7), (8, 10)], intersect((0, 10), (0, 7)));
        assert_eq!(vec![(-5, -1), (0, 7), (8, 10)], intersect((0, 10), (-5, 7)));
        assert_eq!(vec![(-5, -1), (0, 0), (1, 10)], intersect((0, 10), (-5, 0)));
    }
}
