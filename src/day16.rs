use std::io::BufRead;
use std::time::{Duration, Instant};

const BITS: [[u8; 4]; 16] = [
    [0, 0, 0, 0],
    [0, 0, 0, 1],
    [0, 0, 1, 0],
    [0, 0, 1, 1],
    [0, 1, 0, 0],
    [0, 1, 0, 1],
    [0, 1, 1, 0],
    [0, 1, 1, 1],
    [1, 0, 0, 0],
    [1, 0, 0, 1],
    [1, 0, 1, 0],
    [1, 0, 1, 1],
    [1, 1, 0, 0],
    [1, 1, 0, 1],
    [1, 1, 1, 0],
    [1, 1, 1, 1],
];

fn h2b(s: &str) -> Vec<u8> {
    // chars flat_map collect is slower
    let mut v: Vec<u8> = Vec::with_capacity(s.len() * 4);
    for c in s.chars() {
        let i = c as u8;
        let idx = (if i < b'A' { i - b'0' } else { i - b'A' + 10 }) as usize;
        v.extend(BITS[idx]);
    }
    v
}

#[derive(Debug, PartialEq, Clone)]
struct Version(u8);

#[derive(Debug, PartialEq, Clone)]
struct TypeId(u8);

fn read_version(s: &[u8]) -> Option<(Version, &[u8])> {
    Some((Version(decode_bits(s, 3)? as u8), &s[3..]))
}

fn read_type_id(s: &[u8]) -> Option<(TypeId, &[u8])> {
    Some((TypeId(decode_bits(s, 3)? as u8), &s[3..]))
}

#[derive(Debug, PartialEq, Clone)]
enum LengthTypeId {
    TotalLengthOfBits(i64),
    NumberOfSubPackets(i64),
}
use LengthTypeId::*;

fn decode_bits(s: &[u8], n: usize) -> Option<i64> {
    (0..(n as usize)).fold(Some(0), |acc, i| {
        Some(acc? + (1 << (n - i - 1)) * (*s.get(i)? as i64))
    })
}

fn read_length_type_id(s: &[u8]) -> Option<(LengthTypeId, &[u8])> {
    let rest = &s[1..];
    Some(if s[0] == 0 {
        (TotalLengthOfBits(decode_bits(rest, 15)?), &rest[15..])
    } else {
        (NumberOfSubPackets(decode_bits(rest, 11)?), &rest[11..])
    })
}

fn read_number(mut rest: &[u8]) -> Option<(i64, &[u8])> {
    let mut tmp: i64 = 0;
    loop {
        let last = *rest.get(0)? == 0;
        tmp = tmp << 4
            | (*rest.get(1)? as i64) << 3
            | (*rest.get(2)? as i64) << 2
            | (*rest.get(3)? as i64) << 1
            | *rest.get(4)? as i64;
        rest = rest.get(5..)?;
        if last {
            break;
        }
    }
    Some((tmp, rest))
}

#[derive(Debug, PartialEq, Clone)]
enum Packet {
    Literal(Version, i64),
    Operator(Version, TypeId, Vec<Packet>),
}
use Packet::*;

fn parse_packet(s: &[u8]) -> Option<(Packet, &[u8])> {
    let (version, rest) = read_version(s)?;
    let (type_id, rest) = read_type_id(rest)?;
    if type_id == TypeId(4) {
        let (number, rest) = read_number(rest)?;
        return Some((Literal(version, number), rest));
    }
    let (length_type_id, rest) = read_length_type_id(rest)?;
    match length_type_id {
        TotalLengthOfBits(n) => {
            let mut packets = vec![];
            let real_rest = &rest.get(n as usize..)?;
            let mut rest = rest.get(0..n as usize)?;
            while let Some((p, r)) = parse_packet(rest) {
                packets.push(p);
                rest = r;
            }
            Some((Operator(version, type_id, packets), real_rest))
        }
        NumberOfSubPackets(n) => {
            let mut packets = Vec::with_capacity(n as usize);
            let mut rest = rest;
            for _ in 0..n {
                let (p, r) = parse_packet(rest)?;
                packets.push(p);
                rest = r;
            }
            Some((Operator(version, type_id, packets), rest))
        }
    }
}

fn sum_versions(p: &Packet) -> i64 {
    match p {
        Literal(Version(v), _) => *v as i64,
        Operator(Version(v), _, ops) => {
            *v as i64 + ops.iter().map(|p| sum_versions(p)).sum::<i64>()
        }
    }
}

fn eval(p: &Packet) -> i64 {
    match p {
        Literal(_, v) => *v,
        Operator(_, TypeId(0), ops) => ops.iter().map(eval).sum(),
        Operator(_, TypeId(1), ops) => ops.iter().map(eval).product(),
        Operator(_, TypeId(2), ops) => ops.iter().map(eval).min().unwrap(),
        Operator(_, TypeId(3), ops) => ops.iter().map(eval).max().unwrap(),
        Operator(_, TypeId(5), ops) => (eval(&ops[0]) > eval(&ops[1])) as i64,
        Operator(_, TypeId(6), ops) => (eval(&ops[0]) < eval(&ops[1])) as i64,
        Operator(_, TypeId(7), ops) => (eval(&ops[0]) == eval(&ops[1])) as i64,
        p => todo!("unsuported {:?}", p),
    }
}

pub fn solve(input: &mut dyn BufRead, verify_expected: bool, output: bool) -> Duration {
    let input: Vec<String> = input.lines().map(|s| s.unwrap()).collect();

    let s = Instant::now();

    let b = h2b(&input[0]);
    let p = parse_packet(&b);
    let (p, _) = p.unwrap();

    let part1 = sum_versions(&p);
    let part2 = eval(&p);

    let e = s.elapsed();
    if verify_expected {
        assert_eq!(1012, part1);
        assert_eq!(2223947372407, part2);
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

    #[test]
    fn h2b_test() {
        assert_eq!(
            vec![1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0],
            h2b("D2FE28")
        );
    }

    #[test]
    fn parse_literal() {
        let bin = h2b("D2FE28");
        let (version, rest) = read_version(&bin).unwrap();
        assert_eq!(Version(6), version);
        let (type_id, rest) = read_type_id(rest).unwrap();
        assert_eq!(TypeId(4), type_id);
        let (number, rest) = read_number(rest).unwrap();
        assert_eq!(2021, number);
        assert_eq!(vec![0, 0, 0], rest);
    }

    #[test]
    fn parse_total_len() {
        let bin = h2b("38006F45291200");
        let (version, rest) = read_version(&bin).unwrap();
        assert_eq!(Version(1), version);
        let (type_id, rest) = read_type_id(rest).unwrap();
        assert_eq!(TypeId(6), type_id);
        let (length_type_id, rest) = read_length_type_id(rest).unwrap();
        assert_eq!(TotalLengthOfBits(27), length_type_id);
        assert_eq!(
            vec![
                1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0,
                0, 0, 0, 0, 0, 0,
            ],
            rest
        );
    }

    #[test]
    fn parse_number_of_subpackets() {
        let bin = h2b("EE00D40C823060");
        let (version, rest) = read_version(&bin).unwrap();
        assert_eq!(Version(7), version);
        let (type_id, rest) = read_type_id(rest).unwrap();
        assert_eq!(TypeId(3), type_id);
        let (length_type_id, rest) = read_length_type_id(rest).unwrap();
        assert_eq!(NumberOfSubPackets(3), length_type_id);
        assert_eq!(
            vec![
                0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0,
                0, 0, 0, 1, 1, 0, 0, 0, 0, 0
            ],
            rest
        );
    }

    #[test]
    fn test_parse_packet() {
        let b = h2b("D2FE28");
        let (p, r) = parse_packet(&b).unwrap();
        assert_eq!(Literal(Version(6), 2021), p);
        assert_eq!(vec![0, 0, 0], r);

        let b = h2b("38006F45291200");
        let (p, r) = parse_packet(&b).unwrap();
        assert_eq!(
            Operator(
                Version(1),
                TypeId(6),
                vec![Literal(Version(6), 10), Literal(Version(2), 20)]
            ),
            p
        );
        assert_eq!(vec![0, 0, 0, 0, 0, 0, 0], r);

        let b = h2b("EE00D40C823060");
        let (p, r) = parse_packet(&b).unwrap();
        assert_eq!(
            Operator(
                Version(7),
                TypeId(3),
                vec![
                    Literal(Version(2), 1),
                    Literal(Version(4), 2),
                    Literal(Version(1), 3)
                ]
            ),
            p
        );
        assert_eq!(vec![0, 0, 0, 0, 0], r);
    }

    #[test]
    fn other() {
        let (p, _) = parse_packet(&h2b("C200B40A82")).unwrap();
        assert_eq!(3, eval(&p));
        let (p, _) = parse_packet(&h2b("04005AC33890")).unwrap();
        assert_eq!(54, eval(&p));
        let (p, _) = parse_packet(&h2b("880086C3E88112")).unwrap();
        assert_eq!(7, eval(&p));
        let (p, _) = parse_packet(&h2b("CE00C43D881120")).unwrap();
        assert_eq!(9, eval(&p));
        let (p, _) = parse_packet(&h2b("D8005AC2A8F0")).unwrap();
        assert_eq!(1, eval(&p));
        let (p, _) = parse_packet(&h2b("F600BC2D8F")).unwrap();
        assert_eq!(0, eval(&p));
        let (p, _) = parse_packet(&h2b("9C005AC2F8F0")).unwrap();
        assert_eq!(0, eval(&p));
        let (p, _) = parse_packet(&h2b("9C0141080250320F1802104A08")).unwrap();
        assert_eq!(1, eval(&p));
        let (p, _) = parse_packet(&h2b("8A004A801A8002F478")).unwrap();
        assert_eq!(16, sum_versions(&p));
        let (p, _) = parse_packet(&h2b("620080001611562C8802118E34")).unwrap();
        assert_eq!(12, sum_versions(&p));
        let (p, _) = parse_packet(&h2b("C0015000016115A2E0802F182340")).unwrap();
        assert_eq!(23, sum_versions(&p));
        let (p, _) = parse_packet(&h2b("A0016C880162017C3686B18A3D4780")).unwrap();
        assert_eq!(31, sum_versions(&p));
    }
}
