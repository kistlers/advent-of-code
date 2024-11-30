extern crate core;

use elf::SplitExtension;
use itertools::Itertools;

// Code for the problem https://adventofcode.com/2022/day/10
const YEAR: u32 = 2022;
const DAY: u32 = 10;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "<>");

    let mut curr_value: i64 = 1;
    let x = input_lines.iter()
        .flat_map(|input_line| {
            let (len, add): (usize, i64) = if input_line == "noop" {
                (1, 0)
            } else if let Some(("addx", v)) = input_line.split_once_parse_value::<i64>(" ") {
                (2, v)
            } else {
                panic!()
            };
            let next = vec![curr_value; len];
            curr_value += add;
            next
        }).collect_vec();

    let part1 = (20..x.len()).step_by(40)
        .map(|cycle| cycle as i64 * x[cycle]).sum::<i64>();
    println!("part1: {part1}");

    let rows: usize = 6;
    let cols: usize = 40;
    for row in 0..rows {
        for col in 0..cols {
            let sprite_pos = x[row * cols + col];
            if col as i64 >= sprite_pos - 1 && col as i64 <= sprite_pos + 1 {
                print!("#");
            } else {
                print!(".")
            }
        }
        println!();
    }
    println!("part2: PLULKBZH");
}
