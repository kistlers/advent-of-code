use itertools::Itertools;

// Code for the problem https://adventofcode.com/2022/day/01
const YEAR: u32 = 2022;
const DAY: u32 = 01;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "<>");
    let top_calories = &input_lines
        .join(" ")
        .split("  ")
        .map(|foods| {
            foods
                .split(" ")
                .map(|cal| cal.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .collect::<Vec<i32>>();
    println!("part1: {}", top_calories[0]);
    println!("part2: {}", top_calories.iter().sum::<i32>());
}
