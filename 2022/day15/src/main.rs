use std::borrow::BorrowMut;
use std::convert::TryInto;
use elf::measure;
use elf::regex::Regex;
use itertools::Itertools;
use lazy_static::lazy_static;
use geos::geo_types::{LineString, Polygon, Point};
use geos::Geom;

// Code for the problem https://adventofcode.com/2022/day/15
const YEAR: u32 = 2022;
const DAY: u32 = 15;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "<>");

    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
    }

    let (part1, part2) = measure!({
    let sensors_and_beacons = input_lines.iter()
        .map(|input_line|
            if let
                Some((sx, sy, bx, by)) = REGEX.captures(input_line).unwrap().iter().skip(1).map(|num| num.unwrap().as_str().parse::<i64>().unwrap()).collect_tuple()
            { ((sx, sy), (bx, by), manhattan(sx, sy, bx, by)) } else { panic!() }
        ).collect_vec();

    let beacons = sensors_and_beacons.iter()
        .map(|(_, b, _)| b)
        .unique()
        .map(|(bx, by)| (*bx as f64, *by as f64))
        .map(|(bx, by)| {
            let point = Point::from((bx, by));
            let p: geos::Geometry = (&point).try_into().unwrap();
            p
        }).collect_vec();

    let polygons = sensors_and_beacons.iter()
        .map(|((sx, sy), _, distance)| ((*sx as f64, *sy as f64), *distance as f64))
        .map(|((sx, sy), distance)| {
            // need to padding 0.5 in all directions as I am not solving on the discrete integer grid
            let polygon = Polygon::new(
                LineString::from(vec![(sx, sy - distance - 0.5), (sx - distance - 0.5, sy), (sx, sy + distance + 0.5), (sx + distance + 0.5, sy)]),
                vec![],
            );
            let p: geos::Geometry = (&polygon).try_into().unwrap();
            p
        }).collect_vec();
    let covered = polygons.iter().skip(1).fold(Geom::clone(&polygons[0]), |mut acc, p| acc.borrow_mut().union(p).unwrap());

    // the row for part1 is at a different place for the sample input which has less than 20 input lines,
    // differentiated by the magnitude of the first coordinate
    let part1_line_y = if sensors_and_beacons.first().unwrap().0.0.abs() < 1_000 { 10. } else { 2_000_000. };
    let part1_line: geos::Geometry = (&LineString::from(vec![(covered.get_x_min().unwrap(), part1_line_y), (covered.get_x_max().unwrap(), part1_line_y)])).try_into().unwrap();
    let part1_intersection_line = covered.intersection(&part1_line).unwrap();
    // println!("{}", part1_intersection_line.to_wkt().unwrap());
    let part1 = part1_intersection_line.length().unwrap().round() as i64 - beacons.iter().filter(|b| b.intersects(&part1_intersection_line).unwrap()).count() as i64;

    let part2_coord = covered.get_interior_ring_n(0).unwrap().get_centroid().unwrap();
    let part2 = part2_coord.get_x().unwrap().round() as i64 * 4_000_000 + part2_coord.get_y().unwrap().round() as i64;

    (part1, part2)
    });

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}


fn manhattan(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    (x1 - x2).abs() + (y1 - y2).abs()
}


