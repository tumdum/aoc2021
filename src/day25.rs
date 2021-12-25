use std::io::BufRead;
use std::time::{Duration, Instant};

type M = [[u8;139]; 137];

fn canonaical(m: &M, (mut x, mut y): (i32,i32)) -> (i32,i32) {
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

fn get(m: &M, (x, y): (i32,i32)) -> u8 {
    let (x,y) = canonaical(m, (x,y));
    m[y as usize][x as usize]
}

#[allow(dead_code)]
fn print(m: &M) {
    let h = m.len() as i32;
    let w = m[0].len() as i32;

    for y in 0..h {
        for x in 0..w {
            print!("{}", get(m, (x,y)));
        }
        println!()
    }

}

fn next_pos(m: &M, (x,y): (i32,i32)) -> Option<(i32,i32)> {
    let c = get(m, (x,y));
    match c {
        b'>' => Some(canonaical(m, (x+1, y))),
        b'v' => Some(canonaical(m, (x, y+1))),
        _ => None,
    }
}

fn step(m: &M) -> M {
    let h = m.len() as i32;
    debug_assert_eq!(137,h);
    let w = m[0].len() as i32;
    debug_assert_eq!(139,w);
    let mut ret = [[b'.';139]; 137];

    for y in 0..h {
        for x in 0..w {
            let c = get(&m,(x,y));
            if c == b'v' {
                ret[y as usize][x as usize] = b'v';
            }
            if c != b'>' {
                continue;
            }
            let next = next_pos(&m, (x,y)).unwrap();
            let next_c = get(&m, next);
            if next_c == b'.' {
                ret[next.1 as usize][next.0 as usize] = c;
                ret[y as usize][x as usize] = b'.';
            } else {
                ret[y as usize][x as usize] = c;
            }
        }
    }
    
    let m = &ret;
    let mut ret = m.clone();

    for y in 0..h {
        for x in 0..w {
            let c = get(&m,(x,y));
            if c != b'v' {
                continue;
            }
            let next = next_pos(&m, (x,y)).unwrap();
            let next_c = get(&m, next);
            if next_c == b'.' {
                ret[next.1 as usize][next.0 as usize] = c;
                ret[y as usize][x as usize] = b'.';
            } else {
                ret[y as usize][x as usize] = c;
            }
        }
    }

    ret
}

pub fn solve(input: &mut dyn BufRead, verify_expected: bool, output: bool) -> Duration {
    let input: Vec<Vec<char>> = input.lines().map(|s| s.unwrap().chars().collect()).collect();
    let mut map: M = [[b'.'; 139]; 137];
    for y in 0..137 {
        for x in 0..139 {
            map[y][x] = input[y][x] as u8;
        }
    }

    let s = Instant::now();

    let mut part1 = 0;
    for s in 1.. {
        let next = step(&map);
        if next == map {
            break;
        }
        part1 = s + 1;
        map = next;
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

