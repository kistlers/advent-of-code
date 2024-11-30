use std::collections::HashSet;


// Code for the problem https://adventofcode.com/2022/day/03
const YEAR: u32 = 2022;
const DAY: u32 = 03;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "<>");
    let part1 = input_lines.iter()
        .map(|input_line| {
            let (left, right) = input_line.split_at(input_line.len() / 2);
            let left_chars = left.chars().collect::<HashSet<char>>();
            let right_chars = right.chars().collect::<HashSet<char>>();
            let intersection = *(&left_chars & &right_chars).into_iter().collect::<Vec<char>>().first().unwrap();
            priority(intersection)
        }).sum::<u32>();
    println!("part1: {part1}");

    let part2 = input_lines.chunks(3)
        .map(|chunk| {
            let packs = chunk.iter()
                .map(|c| c.chars().collect::<HashSet<char>>()).collect::<Vec<HashSet<char>>>();
            let intersection = *(&(&packs[0] & &packs[1]) & &packs[2]).into_iter().collect::<Vec<char>>().first().unwrap();
            priority(intersection)
        }).sum::<u32>();
    println!("part2: {part2}");
}

fn priority(c: char) -> u32 {
    if c.is_uppercase() { c as u32 - 'A' as u32 + 27 } else { c as u32 - 'a' as u32 + 1 }
}
