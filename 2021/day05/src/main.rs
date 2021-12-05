use elf::SplitExtension;
use std::cmp::max;
use itertools::sorted;

// Code for the problem https://adventofcode.com/2021/day/5
const YEAR: u32 = 2021;
const DAY: u32 = 5;

fn main() {
    let coords: Vec<_> = elf::get_input(YEAR, DAY, "53616c7465645f5fc3caba04b37cfdc549d664f17097aa5457af16a74fbc99744bebc15d2bc22c9784c1224456ac0746").iter().map(|input_line: &String| {
        let (from, to) = input_line.split_once(" -> ").unwrap();
        let (x1, y1): (usize, usize) = from.split_once_parse(",").unwrap();
        let (x2, y2): (usize, usize) = to.split_once_parse(",").unwrap();
        // println!("({}, {}) -> ({}, {})", x1, y1, x2, y2);
        return ((x1, y1), (x2, y2));
    }).collect();

    let (x_max, y_max) = coords.iter().fold((0usize, 0usize), |(x_max, y_max), ((x1, y1), (x2, y2))| (max(max(*x1 as usize, *x2 as usize), x_max), max(max(*y1 as usize, *y2 as usize), y_max)));
    // println!("({}, {})", x_max, y_max);

    let mut field = vec![vec![0; y_max + 1]; x_max + 1];
    coords.iter().for_each(|((x1, y1), (x2, y2))| {
        // println!("({}, {}) -> ({}, {})", x1, y1, x2, y2);
        if *x1 == *x2 {
            let sorted: Vec<_> = sorted([*y1, *y2]).collect();
            // println!("*x1 == *x2: ({}, {}) -> ({}, {})", x1, y1, x2, y2);
            for y in sorted[0]..(sorted[1] + 1) {
                field[*x1][y] += 1;
                // println!("field[{}][{}] = {}", *x1, y, field[*x1][y]);
            }
        } else if *y1 == *y2 {
            let sorted: Vec<_> = sorted([*x1, *x2]).collect();
            // println!("*y1 == *y2: ({}, {}) -> ({}, {})", x1, y1, x2, y2);
            for x in sorted[0]..(sorted[1] + 1) {
                field[x][*y1] += 1;
                // println!("field[{}][{}] = {}", x, *y1, field[x][*y1]);
            }
        }
    });
    let count = field.into_iter().flatten().filter(|x| {
        // print!("{} ", x);
        *x > 1
    }).count();
    println!("Part1: {}", count);
}
