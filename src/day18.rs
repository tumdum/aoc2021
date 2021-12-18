use std::fmt::{Debug, Formatter};
use std::io::BufRead;
use std::rc::Rc;
use std::time::{Duration, Instant};

enum Num {
    Node(Rc<Num>, Rc<Num>),
    Leaf(i64),
}

impl Num {
    fn to_val(&self) -> Option<i64> {
        if let Num::Leaf(n) = self {
            Some(*n)
        } else {
            None
        }
    }
}

impl Debug for Num {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Num::Leaf(n) => write!(f, "{}", n),
            Num::Node(l, r) => {
                write!(f, "[")?;
                l.fmt(f)?;
                write!(f, ",")?;
                r.fmt(f)?;
                write!(f, "]")
            }
        }
    }
}

fn parse(s: &[char]) -> (Rc<Num>, &[char]) {
    if s[0] == '[' {
        let (l, rest) = parse(&s[1..]);
        assert_eq!(rest[0], ',');
        let (r, rest) = parse(&rest[1..]);
        assert_eq!(rest[0], ']');
        (Rc::new(Num::Node(l, r)), &rest[1..])
    } else {
        let start = 0;
        let mut end = 0;
        for i in 0..s.len() {
            if s[i].is_ascii_digit() {
                end = i;
            } else {
                break;
            }
        }
        let sub = s[start..=end].iter().collect::<String>();
        (Rc::new(Num::Leaf(sub.parse().unwrap())), &s[end + 1..])
    }
}

fn nested(n: &Rc<Num>, d: usize) -> Option<(Option<Rc<Num>>, Option<i64>, Option<i64>)> {
    match (&**n, d) {
        (Num::Node(l, r), v) if v < 4 => {
            if let Some((new_left, left, right)) = nested(&l, &d + 1) {
                let (new_r, _) = add_to_left(r, right);
                let node = if let Some(new_left) = new_left {
                    Rc::new(Num::Node(new_left, new_r))
                } else {
                    Rc::new(Num::Node(Rc::new(Num::Leaf(0)), new_r))
                };
                return Some((Some(node), left, None));
            }
            if let Some((new_right, left, right)) = nested(&r, &d + 1) {
                let (new_l, _) = add_to_right(l, left);
                let node = if let Some(new_right) = new_right {
                    Rc::new(Num::Node(new_l, new_right))
                } else {
                    Rc::new(Num::Node(new_l, Rc::new(Num::Leaf(0))))
                };
                return Some((Some(node), None, right));
            }
            return None;
        }
        (Num::Node(l, r), 4) => Some((None, l.to_val(), r.to_val())),
        _ => None,
    }
}

fn split(n: &Rc<Num>) -> Option<Rc<Num>> {
    match &**n {
        Num::Leaf(v) => {
            if *v >= 10 {
                let l = (*v as f64 / 2.0).floor() as i64;
                let r = (*v as f64 / 2.0).ceil() as i64;

                Some(Rc::new(Num::Node(
                    Rc::new(Num::Leaf(l)),
                    Rc::new(Num::Leaf(r)),
                )))
            } else {
                None
            }
        }
        Num::Node(l, r) => {
            if let Some(new_l) = split(l) {
                return Some(Rc::new(Num::Node(new_l, r.clone())));
            }
            if let Some(new_r) = split(r) {
                return Some(Rc::new(Num::Node(l.clone(), new_r)));
            }
            None
        }
    }
}

fn add(l: &Rc<Num>, r: &Rc<Num>) -> Rc<Num> {
    let mut ret = Rc::new(Num::Node(l.clone(), r.clone()));

    loop {
        if let Some((Some(n), _, _)) = nested(&ret, 0) {
            ret = n;
            continue;
        }
        if let Some(n) = split(&ret) {
            ret = n;
            continue;
        }
        break;
    }

    ret
}

fn sum(nums: &[Rc<Num>]) -> Rc<Num> {
    let mut ret = nums[0].clone();
    for i in 1..nums.len() {
        let ret2 = add(&ret, &nums[i]);
        ret = ret2;
    }
    ret
}

fn add_to_left(n: &Rc<Num>, v: Option<i64>) -> (Rc<Num>, bool) {
    match v {
        None => (n.clone(), false),
        Some(v) => match &**n {
            Num::Leaf(n) => (Rc::new(Num::Leaf(n + v)), true),
            Num::Node(l, r) => {
                let (new_l, did_l) = add_to_left(l, Some(v));
                if did_l {
                    return (Rc::new(Num::Node(new_l, r.clone())), true);
                } else {
                    return (
                        Rc::new(Num::Node(l.clone(), add_to_left(r, Some(v)).0)),
                        true,
                    );
                }
            }
        },
    }
}

fn magnitude(n: &Rc<Num>) -> i64 {
    match &**n {
        Num::Leaf(v) => *v,
        Num::Node(l, r) => 3 * magnitude(l) + 2 * magnitude(r),
    }
}

fn add_to_right(n: &Rc<Num>, v: Option<i64>) -> (Rc<Num>, bool) {
    match v {
        None => (n.clone(), false),
        Some(v) => match &**n {
            Num::Leaf(n) => (Rc::new(Num::Leaf(n + v)), true),
            Num::Node(l, r) => {
                let (new_r, did_r) = add_to_right(r, Some(v));
                if did_r {
                    return (Rc::new(Num::Node(l.clone(), new_r)), true);
                } else {
                    return (
                        Rc::new(Num::Node(add_to_right(l, Some(v)).0, r.clone())),
                        true,
                    );
                }
            }
        },
    }
}

pub fn solve(input: &mut dyn BufRead, verify_expected: bool, output: bool) -> Duration {
    let input: Vec<Rc<Num>> = input
        .lines()
        .map(|s| parse(&s.unwrap().chars().collect::<Vec<_>>()).0)
        .collect();

    let s = Instant::now();

    let part1 = magnitude(&sum(&input));

    let mut part2 = 0;
    for i in 0..input.len() {
        for j in 0..input.len() {
            if i == j {
                continue;
            }
            let ij = magnitude(&add(&input[i], &input[j]));
            if part2 < ij {
                part2 = ij;
            }
            let ji = magnitude(&add(&input[j], &input[i]));
            if part2 < ji {
                part2 = ji;
            }
        }
    }

    let e = s.elapsed();
    if verify_expected {
        assert_eq!(2501, part1);
        assert_eq!(4935, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    e
}

#[cfg(test)]
mod tests {
    use super::*;
    fn validate_nested(input: &str, expected: &str) {
        let (n, _) = parse(&input.chars().collect::<Vec<_>>());
        let (n, _, _) = nested(&n, 0).unwrap();
        assert_eq!(expected, format!("{:?}", n.unwrap()));
    }

    fn validate_sum(input: Vec<&str>, expected: &str) {
        let nums: Vec<Rc<Num>> = input
            .iter()
            .map(|s| parse(&s.chars().collect::<Vec<_>>()).0)
            .collect();
        assert_eq!(expected, format!("{:?}", sum(&nums)));
    }

    #[test]
    fn from_description() {
        dbg!(parse(&"123".chars().collect::<Vec<_>>()));
        dbg!(parse(&"[4,7]".chars().collect::<Vec<_>>()));
        let (n, _) = dbg!(parse(&"[[[[[9,8],1],2],3],4]".chars().collect::<Vec<_>>()));

        dbg!(nested(&n, 0));

        validate_nested("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
        validate_nested("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
        validate_nested("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
        validate_nested(
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        );
        validate_nested(
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        );

        validate_sum(
            vec!["[1,1]", "[2,2]", "[3,3]", "[4,4]"],
            "[[[[1,1],[2,2]],[3,3]],[4,4]]",
        );
        validate_sum(
            vec!["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]"],
            "[[[[3,0],[5,3]],[4,4]],[5,5]]",
        );
        validate_sum(
            vec!["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"],
            "[[[[5,0],[7,4]],[5,5]],[6,6]]",
        );
        println!();
        println!();
        validate_sum(
            vec![
                "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
                "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
                "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
                "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
                "[7,[5,[[3,8],[1,4]]]]",
                "[[2,[2,2]],[8,[8,1]]]",
                "[2,9]",
                "[1,[[[9,3],9],[[9,0],[0,7]]]]",
                "[[[5,[7,4]],7],1]",
                "[[[[4,2],2],6],[8,7]]",
            ],
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        );
    }
}
