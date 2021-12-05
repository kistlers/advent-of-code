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
        return ((x1, y1), (x2, y2));
    }).collect();

    let (x_max, y_max) = coords.iter().fold((0usize, 0usize), |(x_max, y_max), ((x1, y1), (x2, y2))| (max(max(*x1 as usize, *x2 as usize), x_max), max(max(*y1 as usize, *y2 as usize), y_max)));

    let mut field = vec![vec![0; y_max + 1]; x_max + 1];
    coords.iter().for_each(|((x1, y1), (x2, y2))| {
        if *x1 == *x2 {
            let sorted: Vec<_> = sorted([*y1, *y2]).collect();
            for y in sorted[0]..(sorted[1] + 1) {
                field[*x1][y] += 1;
            }
        } else if *y1 == *y2 {
            let sorted: Vec<_> = sorted([*x1, *x2]).collect();
            for x in sorted[0]..(sorted[1] + 1) {
                field[x][*y1] += 1;
            }
        }
    });
    let count = field.into_iter().flatten().filter(|x| { *x > 1 }).count();
    println!("Part1: {}", count);

    let mut field = vec![vec![0; y_max + 1]; x_max + 1];
    coords.iter().for_each(|((x1, y1), (x2, y2))| {
        let x_from = *x1 as i32;
        let y_from = *y1 as i32;
        let x_step: i32;
        let y_step: i32;
        let x_len: i32;
        let y_len: i32;

        if *x1 == *x2 {
            x_len = 1;
            x_step = 0;
        } else if *x1 > *x2 {
            x_len = (*x1 - *x2 + 1) as i32;
            x_step = -1;
        } else {
            x_len = (*x2 - *x1 + 1) as i32;
            x_step = 1;
        }

        if *y1 == *y2 {
            y_len = 1;
            y_step = 0;
        } else if *y1 > *y2 {
            y_len = (*y1 - *y2 + 1) as i32;
            y_step = -1;
        } else {
            y_len = (*y2 - *y1 + 1) as i32;
            y_step = 1;
        }

        for i in 0..max(x_len, y_len) {
            let x = (x_from + (x_step) as i32 * i) as usize;
            let y = (y_from + (y_step) as i32 * i) as usize;
            field[x][y] += 1;
        }
    });
    let count = field.into_iter().flatten().filter(|x| { *x > 1 }).count();
    println!("Part2: {}", count);
}
