use std::fmt::Error;
use std::str::FromStr;
use elf::SplitExtension;

// Code for the problem https://adventofcode.com/2022/day/04
const YEAR: u32 = 2022;
const DAY: u32 = 04;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "<>");
    let range_pairs = input_lines.iter()
        .map(|input_line| input_line.parse::<RangePair>().unwrap())
        .collect::<Vec<RangePair>>();

    let part1 = range_pairs.iter()
        .filter(|range_pair| range_pair.full_overlap())
        .count();
    println!("part1: {part1}");

    let part2 = range_pairs.iter()
        .filter(|range_pair| range_pair.overlap())
        .count();
    println!("part2: {part2}");
}

#[derive(Debug, Clone)]
struct Range {
    from: usize,
    to: usize,
}

impl Range {
    pub(crate) fn contains(&self, other: &Range) -> bool {
        self.from <= other.from && other.to <= self.to
    }
}

impl Range {
    fn new(from: usize, to: usize) -> Range {
        Self { from, to }
    }
}

impl FromStr for Range {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to): (usize, usize) = s.split_once_parse("-").unwrap();
        Ok(Range::new(from, to))
    }
}

#[derive(Debug, Clone)]
struct RangePair {
    left: Range,
    right: Range,
}

impl RangePair {
    pub(crate) fn full_overlap(&self) -> bool {
        self.left.contains(&self.right) || self.right.contains(&self.left)
    }
}

impl RangePair {
    pub(crate) fn overlap(&self) -> bool {
        self.left.from <= self.right.to && self.right.from <= self.left.to
    }
}

impl RangePair {
    fn new(left: Range, right: Range) -> RangePair {
        Self { left, right }
    }
}

impl FromStr for RangePair {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right): (Range, Range) = s.split_once_parse(",").unwrap();
        Ok(RangePair::new(left, right))
    }
}