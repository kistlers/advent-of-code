use std::cmp;
use elf::measure;
use elf::regex::{Captures, Regex};
use itertools::Itertools;

// Code for the problem https://adventofcode.com/2021/day/22
const YEAR: u32 = 2021;
const DAY: u32 = 22;

const PART1_MIN: i64 = -50;
const PART1_MAX: i64 = 50;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "53616c7465645f5fc3caba04b37cfdc549d664f17097aa5457af16a74fbc99744bebc15d2bc22c9784c1224456ac0746");

    let part1_count;
    let part2_count;
    measure!({
        let regex = Regex::new(r"(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)").unwrap();

        let mut on_regions: Vec<Region> = vec![];
        for input_line in input_lines {
            let (is_on, region) = parse_region(&regex.captures(&input_line).unwrap());
            on_regions.iter_mut().for_each(|r| r.subtract(&region));
            if is_on {
                on_regions.push(region);
            }
        }
        part1_count = on_regions.iter().map(|r| r.volume_limit_part1()).sum::<i64>();
        part2_count = on_regions.iter().map(|r| r.volume()).sum::<i64>();
    });
    println!("Part1: {}", part1_count);
    println!("Part2: {}", part2_count);
}

fn parse_region(captures: &Captures) -> (bool, Region) {
    let is_on = captures.get(1).unwrap().as_str() == "on";
    let (x_from, x_to, y_from, y_to, z_from, z_to) = captures
        .iter()
        .skip(2)
        .map(|m| m.unwrap().as_str().parse::<i64>().unwrap())
        .collect_tuple()
        .unwrap();
    (is_on, Region::new(x_from, x_to, y_from, y_to, z_from, z_to))
}

#[derive(Debug, Clone)]
struct Region {
    x_from: i64,
    x_to: i64,
    y_to: i64,
    y_from: i64,
    z_from: i64,
    z_to: i64,
    off_regions: Vec<Region>,
}

impl Region {
    fn new(x_from: i64, x_to: i64, y_from: i64, y_to: i64, z_from: i64, z_to: i64) -> Self {
        Self { x_from, x_to, y_from, y_to, z_from, z_to, off_regions: vec![] }
    }

    fn subtract(&mut self, other: &Region) {
        if self.intersects(other) {
            let (x_from, x_to) = interval_overlap(self.x_from, self.x_to, other.x_from, other.x_to);
            let (y_from, y_to) = interval_overlap(self.y_from, self.y_to, other.y_from, other.y_to);
            let (z_from, z_to) = interval_overlap(self.z_from, self.z_to, other.z_from, other.z_to);
            self.off_regions.iter_mut().for_each(|r| r.subtract(other));
            self.off_regions.push(Region::new(x_from, x_to, y_from, y_to, z_from, z_to));
        }
    }

    fn volume(&self) -> i64 {
        (self.x_to - self.x_from + 1)
            * (self.y_to - self.y_from + 1)
            * (self.z_to - self.z_from + 1)
            - self.off_regions.iter().map(|r| r.volume()).sum::<i64>()
    }

    fn volume_limit_part1(&self) -> i64 {
        if self.x_to > PART1_MAX || self.x_from < PART1_MIN || self.y_to > PART1_MAX || self.y_from < PART1_MIN || self.z_to > PART1_MAX || self.z_from < PART1_MIN {
            return 0;
        }

        (limit_part1(self.x_to) - limit_part1(self.x_from) + 1)
            * (limit_part1(self.y_to) - limit_part1(self.y_from) + 1)
            * (limit_part1(self.z_to) - limit_part1(self.z_from) + 1)
            - self.off_regions.iter().map(|r| r.volume_limit_part1()).sum::<i64>()
    }

    fn intersects(&self, other: &Region) -> bool {
        interval_overlaps(self.x_from, self.x_to, other.x_from, other.x_to) &&
            interval_overlaps(self.y_from, self.y_to, other.y_from, other.y_to) &&
            interval_overlaps(self.z_from, self.z_to, other.z_from, other.z_to)
    }
}

fn limit_part1(num: i64) -> i64 {
    num::clamp(num, PART1_MIN, PART1_MAX)
}

fn interval_overlaps(a_from: i64, a_to: i64, b_from: i64, b_to: i64) -> bool {
    (a_from <= b_to) && (b_from <= a_to)
}

fn interval_overlap(a_from: i64, a_to: i64, b_from: i64, b_to: i64) -> (i64, i64) {
    (cmp::max(a_from, b_from), cmp::min(a_to, b_to))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval_overlaps() {
        // full overlap
        assert!(interval_overlaps(3, 7, 5, 9));
        assert!(interval_overlaps(3, 7, 1, 5));
        assert!(interval_overlaps(3, 7, 4, 6));
        assert!(interval_overlaps(3, 7, 3, 7));

        // edge overlap
        assert!(interval_overlaps(3, 7, 0, 3));
        assert!(interval_overlaps(3, 7, 7, 10));

        // no overlap
        assert!(!interval_overlaps(3, 7, 0, 2));
        assert!(!interval_overlaps(3, 7, 8, 10));
    }

    #[test]
    fn test_interval_overlap() {
        // full overlap
        assert_eq!(interval_overlap(3, 7, 5, 9), (5, 7));
        assert_eq!(interval_overlap(3, 7, 1, 5), (3, 5));
        assert_eq!(interval_overlap(3, 7, 4, 6), (4, 6));
        assert_eq!(interval_overlap(3, 7, 3, 7), (3, 7));

        // edge overlap
        assert_eq!(interval_overlap(3, 7, 0, 3), (3, 3));
        assert_eq!(interval_overlap(3, 7, 7, 10), (7, 7));
    }
}
