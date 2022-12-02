use std::collections::HashMap;
use elf::SplitExtension;

// Code for the problem https://adventofcode.com/2022/day/02
const YEAR: u32 = 2022;
const DAY: u32 = 02;

// Part 1
// A, X Rock
// B, Y Paper
// C, Z Scissors

// Part 2
// X lose
// Y draw
// Z win


fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "53616c7465645f5f235e9a21eb1ad1558a877268d5a6533d8a3c56c539f0886b51e27e5ccf230b8815cb5ca121c1f3173a16dd022c3803eefcd5e6fb5f077dae");
    let inputs = &input_lines.iter().map(|input_line| {
        input_line.split_once_parse(" ").unwrap()
    }).collect::<Vec<(char, char)>>();

    let score_part_1 = HashMap::from([
        ('A', HashMap::from([
            ('X', 4),
            ('Y', 8),
            ('Z', 3)
        ])),
        ('B', HashMap::from([
            ('X', 1),
            ('Y', 5),
            ('Z', 9)
        ])),
        ('C', HashMap::from([
            ('X', 7),
            ('Y', 2),
            ('Z', 6)
        ]))
    ]);
    let score_part_2 = HashMap::from([
        ('A', HashMap::from([
            ('X', 3),
            ('Y', 4),
            ('Z', 8)
        ])),
        ('B', HashMap::from([
            ('X', 1),
            ('Y', 5),
            ('Z', 9)
        ])),
        ('C', HashMap::from([
            ('X', 2),
            ('Y', 6),
            ('Z', 7)
        ]))]);

    let part1 = &inputs.iter().map(|(opponent, me)| score_part_1[opponent][me]).sum::<u32>();
    println!("part1: {}", part1);

    let part2 = &inputs.iter().map(|(opponent, me)| score_part_2[opponent][me]).sum::<u32>();
    println!("part2: {}", part2);
}