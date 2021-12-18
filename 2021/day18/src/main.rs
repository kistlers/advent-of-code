use std::fmt::{Debug, Display, Formatter};
use std::ops::Add;
use elf::measure;
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::map;
use nom::IResult;
use nom::sequence::{delimited, separated_pair};
use itertools::Itertools;

// Code for the problem https://adventofcode.com/2021/day/18
const YEAR: u32 = 2021;
const DAY: u32 = 18;

type OptionOfOptionPair<T> = Option<(Option<T>, Option<T>)>;

#[derive(Debug, Clone)]
enum Snailfish {
    Value(u64),
    Pair(Box<Snailfish>, Box<Snailfish>),
}

impl Snailfish {}

impl Snailfish {
    fn magnitude(&self) -> u64 {
        match self {
            Self::Value(value) => *value,
            Self::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude()
        }
    }

    fn reduce(self) -> Self {
        let mut snailfish = self;
        loop {
            let (next, result) = snailfish.explode(0);
            snailfish = next;
            if result.is_some() {
                continue;
            }

            let (next, did_split) = snailfish.split();
            snailfish = next;
            if !did_split {
                break;
            }
        }
        snailfish
    }

    fn explode(self, depth: usize) -> (Self, OptionOfOptionPair<u64>) {
        match self {
            Self::Value(_) => (self, None),
            Self::Pair(left, right) => match (*left, *right) {
                (Self::Value(value_left), Self::Value(value_right)) if depth == 4 =>
                    (Self::Value(0), Some((Some(value_left), Some(value_right)))),
                (left, right) =>
                    match left.explode(depth + 1) {
                        (left_exploded, Some((left_to_add, right_to_add))) => {
                            let right_added = if let Some(right_to_add) = right_to_add {
                                right.add_to_leftmost(right_to_add)
                            } else {
                                right
                            };
                            (
                                Self::Pair(Box::new(left_exploded), Box::new(right_added)),
                                Some((left_to_add, None))
                            )
                        }
                        (left_exploded, None) => match right.explode(depth + 1) {
                            (right_exploded, Some((left_to_add, right_to_add))) => {
                                let left_added = if let Some(left_to_add) = left_to_add {
                                    left_exploded.add_to_rightmost(left_to_add)
                                } else {
                                    left_exploded
                                };
                                (
                                    Self::Pair(Box::new(left_added), Box::new(right_exploded)),
                                    Some((None, right_to_add))
                                )
                            }
                            (right_exploded, None) =>
                                (Self::Pair(Box::new(left_exploded), Box::new(right_exploded)), None)
                        }
                    }
            }
        }
    }

    fn split(self) -> (Self, bool) {
        match self {
            Self::Value(value) if value >= 10 =>
                (Self::Pair(
                    Box::new(Self::Value(value / 2)),
                    Box::new(Self::Value(f64::ceil(value as f64 / 2f64) as u64)),
                ), true),
            Self::Value(_) => (self, false),
            Self::Pair(left, right) => {
                let (left_split, left_did_split) = left.split();
                if left_did_split {
                    (Self::Pair(Box::new(left_split), right), true)
                } else {
                    let (right_split, right_did_split) = right.split();
                    (Self::Pair(Box::new(left_split), Box::new(right_split)), right_did_split)
                }
            }
        }
    }

    fn add_to_rightmost(self, to_add: u64) -> Self {
        match self {
            Self::Value(value) => Self::Value(value + to_add),
            Self::Pair(left, right) => Self::Pair(left, Box::new(right.add_to_rightmost(to_add)))
        }
    }

    fn add_to_leftmost(self, to_add: u64) -> Self {
        match self {
            Self::Value(value) => Self::Value(value + to_add),
            Self::Pair(left, right) => Self::Pair(Box::new(left.add_to_leftmost(to_add)), right)
        }
    }
}

impl Display for Snailfish {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(value) => write!(f, "{}", *value),
            Self::Pair(left, right) => write!(f, "[{},{}]", left, right),
        }
    }
}

impl Add for Snailfish {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Pair(Box::new(self), Box::new(rhs))
    }
}

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "53616c7465645f5fc3caba04b37cfdc549d664f17097aa5457af16a74fbc99744bebc15d2bc22c9784c1224456ac0746");

    let magnitude_sum: u64;
    let biggest_magnitude: u64;
    measure!({
    let snailfishes = input_lines
        .iter()
        .map(|l| parse_snailfish(l).unwrap().1)
        .collect::<Vec<_>>();

    magnitude_sum = snailfishes
        .clone()
        .into_iter()
        .reduce(|left, right| (left + right).reduce())
        .unwrap()
        .magnitude();

    biggest_magnitude = snailfishes
        .into_iter()
        .permutations(2)
        .map(|perm| perm
            .into_iter()
            .reduce(|left, right| (left + right).reduce())
            .unwrap()
            .magnitude()
        )
        .max()
        .unwrap();
    });
    println!("Part1: {}", magnitude_sum);
    println!("Part2: {}", biggest_magnitude);
}

fn parse_snailfish(line: &str) -> IResult<&str, Snailfish> {
    alt((
        map(nom::character::complete::u64, Snailfish::Value),
        map(delimited(
            char('['),
            separated_pair(parse_snailfish, char(','), parse_snailfish),
            char(']'),
        ), |(left, right)| Snailfish::Pair(Box::new(left), Box::new(right))),
    ))(line)
}
