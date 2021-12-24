use std::io::BufRead;
use std::time::{Duration, Instant};

fn digit(mut z: i64, w: i64, off1: i64, off2: i64, off3: i64) -> i64 {
    let x = if (z % 26 + off1) == w { 0 } else { 1 };
    z = z / 26;
    x * (25 * z + w + off3) + z * off2
}

fn find_w_for_digit(z: i64, offset: i64) -> i64 {
    z % 26 + offset
}

fn step1(input: &[i64]) -> i64 {
    let mut z = input[0] + 7;
    z = 26 * z + 8 + input[1];
    z = 26 * z + 16 + input[2];
    z = 26 * z + 8 + input[3];
    z
}

fn to_digits(v: i64) -> Vec<i64> {
    let input: Vec<_> = v
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect();
    input
}

struct State {
    num: i64,
    z: i64,
}

pub fn solve(input: &mut dyn BufRead, verify_expected: bool, output: bool) -> Duration {
    let _progam: Vec<String> = input.lines().map(|s| s.unwrap()).collect();

    let s = Instant::now();

    let mut cands_z = vec![];

    for i in 1000..9999 {
        let digits = to_digits(i);
        if digits.iter().any(|d| *d == 0) {
            continue;
        }
        let s1 = step1(&digits);
        let w = find_w_for_digit(s1, -8);
        if w > 0 && w < 10 {
            let z = digit(s1, w, -8, 1, 3);
            let num = i * 10 + w;
            cands_z.push(State { num, z });
        }
    }

    let mut cands_z2 = Vec::with_capacity(cands_z.len());
    for state in cands_z {
        for i5 in 1..=9 {
            let z = 26 * state.z + 12 + i5;
            let w = find_w_for_digit(z, -11);
            if w > 0 && w < 10 {
                let z = digit(z, w, -11, 1, 1);
                let num = state.num * 100 + 10 * i5 + w;
                cands_z2.push(State { num, z });
            }
        }
    }
    let mut cands_z3 = Vec::with_capacity(cands_z2.len());
    for state in cands_z2 {
        for i5 in 1..=9 {
            let z = 26 * state.z + 8 + i5;
            let w = find_w_for_digit(z, -6);
            if w > 0 && w < 10 {
                let z = digit(z, w, -6, 1, 8);
                let w2 = find_w_for_digit(z, -9);
                if w2 > 0 && w2 < 10 {
                    let z = digit(z, w2, -9, 1, 14);
                    let num = state.num * 1000 + 100 * i5 + 10 * w + w2;
                    cands_z3.push(State { num, z });
                }
            }
        }
    }
    let mut cands_z4 = Vec::with_capacity(cands_z3.len());
    for state in cands_z3 {
        for i5 in 1..=9 {
            let z = 26 * state.z + 4 + i5;
            let w = find_w_for_digit(z, -5);
            if w > 0 && w < 10 {
                let z = digit(z, w, -5, 1, 14);
                let w2 = find_w_for_digit(z, -4);
                if w2 > 0 && w2 < 10 {
                    let z = digit(z, w2, -4, 1, 15);
                    let num = state.num * 1000 + 100 * i5 + 10 * w + w2;
                    cands_z4.push(State { num, z });
                }
            }
        }
    }

    let mut part1 = 0;
    let mut part2 = 99999999999999999;
    for state in cands_z4 {
        let w = find_w_for_digit(state.z, -9);
        if w > 0 && w < 10 {
            let num = state.num * 10 + w;
            if num < part2 {
                part2 = num;
            }
            if num > part1 {
                part1 = num;
            }
        }
    }

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(95299897999897, part1);
        assert_eq!(31111121382151, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    e
}

#[allow(dead_code)]
fn prog(input: &[i64]) -> i64 {
    assert_eq!(14, input.len());

    let mut z = step1(&input);

    z = digit(z, input[4], -8, 1, 3);

    z = 26 * z + 12 + input[5];

    z = digit(z, input[6], -11, 1, 1);

    z = 26 * z + 8 + input[7];

    z = digit(z, input[8], -6, 1, 8);

    z = digit(z, input[9], -9, 1, 14);

    z = 26 * z + 4 + input[10];

    z = digit(z, input[11], -5, 1, 14);

    z = digit(z, input[12], -4, 1, 15);

    z = digit(z, input[13], -9, 1, 6);

    z
}

#[allow(dead_code)]
fn run(p: &[String], digits: &[i64]) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;
    let mut w = 0;
    let mut next_input = 0;
    for inst in p {
        let inst: Vec<_> = inst.split(' ').collect();
        // println!("{:?}", inst);
        match inst[0] {
            "inp" => {
                assert_eq!("w", inst[1]);
                w = digits[next_input];
                next_input += 1;
                println!("{:10} {:10} {:10} {:10}", x, y, z, w);
            }
            "mul" => {
                let v: i64 = match inst[2] {
                    "x" => x,
                    "y" => y,
                    "z" => z,
                    "w" => w,
                    s => s.parse().unwrap(),
                };
                let target = match inst[1] {
                    "x" => &mut x,
                    "y" => &mut y,
                    "z" => &mut z,
                    "w" => &mut w,
                    _ => todo!(),
                };
                *target = *target * v;
            }
            "add" => {
                let v: i64 = match inst[2] {
                    "x" => x,
                    "y" => y,
                    "z" => z,
                    "w" => w,
                    s => s.parse().unwrap(),
                };
                let target = match inst[1] {
                    "x" => &mut x,
                    "y" => &mut y,
                    "z" => &mut z,
                    "w" => &mut w,
                    _ => todo!(),
                };
                *target = *target + v;
            }
            "mod" => {
                let v: i64 = match inst[2] {
                    "x" => x,
                    "y" => y,
                    "z" => z,
                    "w" => w,
                    s => s.parse().unwrap(),
                };
                let target = match inst[1] {
                    "x" => &mut x,
                    "y" => &mut y,
                    "z" => &mut z,
                    "w" => &mut w,
                    _ => todo!(),
                };
                *target = *target % v;
            }
            "div" => {
                let v: i64 = match inst[2] {
                    "x" => x,
                    "y" => y,
                    "z" => z,
                    "w" => w,
                    s => s.parse().unwrap(),
                };
                let target = match inst[1] {
                    "x" => &mut x,
                    "y" => &mut y,
                    "z" => &mut z,
                    "w" => &mut w,
                    _ => todo!(),
                };
                *target = *target / v;
            }
            "eql" => {
                let v: i64 = match inst[2] {
                    "x" => x,
                    "y" => y,
                    "z" => z,
                    "w" => w,
                    s => s.parse().unwrap(),
                };
                let target = match inst[1] {
                    "x" => &mut x,
                    "y" => &mut y,
                    "z" => &mut z,
                    "w" => &mut w,
                    _ => todo!(),
                };
                *target = if *target == v { 1 } else { 0 };
            }
            _ => todo!(),
        }
    }
    z
}
