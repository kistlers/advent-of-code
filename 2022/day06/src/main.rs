use itertools::Itertools;

// Code for the problem https://adventofcode.com/2022/day/06
const YEAR: u32 = 2022;
const DAY: u32 = 06;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "<>");
    let input_line = input_lines.first().unwrap();
    println!("part1: {}", first_all_different(input_line, 4));
    println!("part2: {}", first_all_different(input_line, 14));
}

fn first_all_different(input_line: &str, marker_len: usize) -> usize {
    let mut first_match = marker_len;
    input_line.chars()
        .collect::<Vec<char>>()
        .windows(marker_len)
        .skip_while(|window| if window.iter().unique().count() < window.len() {
            // println!("window")
            first_match += 1;
            true
        } else {
            false
        })
        .count();
    first_match
}
