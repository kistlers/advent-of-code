use std::cmp::Ordering;
use std::fmt::{Display, Error, Formatter};
use std::iter::zip;
use std::str::FromStr;
use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::map;
use nom::IResult;
use nom::multi::separated_list0;
use nom::sequence::{delimited};

// Code for the problem https://adventofcode.com/2022/day/13
const YEAR: u32 = 2022;
const DAY: u32 = 13;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "<>");
    let packets = input_lines.iter()
        .filter(|input_line| !input_line.is_empty())
        .map(|packet| packet.parse::<Packet>().unwrap())
        .collect_vec();

    let part1 = packets
        .chunks(2)
        .map(|packet| (packet[0].clone(), packet[1].clone()))
        .enumerate()
        .filter(|(_, (left, right))| !left.cmp(right).is_gt())
        .filter(|(_, (left, right))| !left.cmp(right).is_gt())
        .map(|(index, _)| index + 1)
        .sum::<usize>();
    println!("part1: {}", part1);

    let divider2 = "[[2]]".parse::<Packet>().unwrap();
    let divider6 = "[[6]]".parse::<Packet>().unwrap();
    let dividers = [&divider2, &divider6];
    let sorted_packets = packets.iter()
        .chain(dividers)
        .sorted()
        .collect_vec();
    let part2 = (sorted_packets.iter().position(|p| **p == divider2).unwrap() + 1)
        * (sorted_packets.iter().position(|p| **p == divider6).unwrap() + 1);
    println!("part2: {}", part2);
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Value(l), Packet::Value(r)) => l.cmp(r),
            (Packet::Value(_), Packet::Separated(_)) => Packet::Separated(Vec::from([self.clone()])).cmp(other),
            (Packet::Separated(_), Packet::Value(_)) => self.cmp(&Packet::Separated(Vec::from([other.clone()]))),
            (Packet::Separated(l), Packet::Separated(r)) => compare_packets(l, r)
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn compare_packets(left: &[Packet], right: &[Packet]) -> Ordering {
    for (l, r) in zip(left, right) {
        let compare = l.cmp(r);
        if compare.is_ne() {
            return compare;
        }
    }
    left.len().cmp(&right.len())
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Value(u64),
    Separated(Vec<Packet>),
}

impl Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(value) => write!(f, "{}", *value),
            Self::Separated(packet) => {
                write!(f, "[")?;
                for l in packet.iter().take(1) {
                    write!(f, "{}", l)?;
                }
                for l in packet.iter().skip(1) {
                    write!(f, ",{}", l)?;
                }
                write!(f, "]")?;
                Ok(())
            }
        }
    }
}

impl FromStr for Packet {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse_packet(s).unwrap().1)
    }
}

fn parse_packet(line: &str) -> IResult<&str, Packet> {
    alt((
        map(nom::character::complete::u64, Packet::Value),
        map(delimited(
            char('['),
            separated_list0(char(','), parse_packet),
            char(']'),
        ), Packet::Separated),
    ))(line)
}

