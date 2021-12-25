use std::io::BufRead;
use std::time::{Duration, Instant};

fn canonaical(m: &[Vec<char>], (mut x, mut y): (i32,i32)) -> (i32,i32) {
    let h = m.len() as i32;
    let w = m[0].len() as i32;
    if x < 0 {
        x += w;
    } else if x >= w {
        x -= w;
    }
    if y < 0 {
        y += h;
    } else if y >= h {
        y -= h;
    }
    (x,y)
}

fn get(m: &[Vec<char>], (x, y): (i32,i32)) -> char {
    let (x,y) = canonaical(m, (x,y));
    m[y as usize][x as usize]
}

#[allow(dead_code)]
fn print(m: &[Vec<char>]) {
    let h = m.len() as i32;
    let w = m[0].len() as i32;

    for y in 0..h {
        for x in 0..w {
            print!("{}", get(m, (x,y)));
        }
        println!()
    }

}

fn next_pos(m: &[Vec<char>], (x,y): (i32,i32)) -> Option<(i32,i32)> {
    let c = get(m, (x,y));
    match c {
        '>' => Some(canonaical(m, (x+1, y))),
        'v' => Some(canonaical(m, (x, y+1))),
        _ => None,
    }
}

fn step(m: &[Vec<char>]) -> Vec<Vec<char>> {
    let h = m.len() as i32;
    let w = m[0].len() as i32;
    let mut ret = vec![vec!['.'; w as usize]; h as usize];

    for y in 0..h {
        for x in 0..w {
            let c = get(&m,(x,y));
            if c == 'v' {
                ret[y as usize][x as usize] = 'v';
            }
            if c != '>' {
                continue;
            }
            let next = next_pos(&m, (x,y)).unwrap();
            let next_c = get(&m, next);
            if next_c == '.' {
                ret[next.1 as usize][next.0 as usize] = c;
                ret[y as usize][x as usize] = '.';
            } else {
                ret[y as usize][x as usize] = c;
            }
        }
    }
    
    let m = &ret;
    let mut ret = m.to_vec();

    for y in 0..h {
        for x in 0..w {
            let c = get(&m,(x,y));
            if c != 'v' {
                continue;
            }
            let next = next_pos(&m, (x,y)).unwrap();
            let next_c = get(&m, next);
            if next_c == '.' {
                ret[next.1 as usize][next.0 as usize] = c;
                ret[y as usize][x as usize] = '.';
            } else {
                ret[y as usize][x as usize] = c;
            }
        }
    }

    ret
}

pub fn solve(input: &mut dyn BufRead, verify_expected: bool, output: bool) -> Duration {
    let mut input: Vec<Vec<char>> = input.lines().map(|s| s.unwrap().chars().collect()).collect();


    let s = Instant::now();

    let mut part1 = 0;
    for s in 1.. {
        let next = step(&input);
        if next == input {
            break;
        }
        part1 = s + 1;
        input = next;
    }


    let e = s.elapsed();
    if verify_expected {
        assert_eq!(429, part1);
    }
    if output {
        println!("\t{}", part1);
    }
    e
}

