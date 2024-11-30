use std::collections::{BTreeMap, HashMap};
use elf::SplitExtension;

// Code for the problem https://adventofcode.com/2021/day/14
const YEAR: u32 = 2021;
const DAY: u32 = 14;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "53616c7465645f5fc3caba04b37cfdc549d664f17097aa5457af16a74fbc99744bebc15d2bc22c9784c1224456ac0746");
    let first_char = input_lines[0].chars().take(1).collect::<Vec<char>>()[0];
    let mut pairs_with_counts = HashMap::new();
    input_lines[0]
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .for_each(|pair| *pairs_with_counts.entry((pair[0], pair[1])).or_insert(0) += 1);
    // println!("{}", sequence);
    let operations = input_lines
        .iter()
        .skip(2)
        .map(|line| line.split_once_parse_value::<char>(" -> ").unwrap())
        .map(|(from, to)| (from.chars().collect::<Vec<_>>(), to))
        .map(|(from, to)| ((from[0], from[1]), to))
        .collect::<HashMap<(char, char), char>>();

    for step in 1..=40 {
        pairs_with_counts = update_counts(&pairs_with_counts, &operations);

        if step == 10 || step == 40 {
            // only count the second char in the pair and the very first of the initial input
            let mut counts = BTreeMap::from([(first_char, 1)]);
            for ((_, p1), count) in pairs_with_counts.iter() {
                *counts.entry(*p1).or_insert(0) += count;
            }
            let max = counts.iter().max_by_key(|&(_, count)| count).unwrap();
            let min = counts.iter().min_by_key(|&(_, count)| count).unwrap();
            if step == 10 {
                println!("Part1: {}", max.1 - min.1);
            }
            if step == 40 {
                println!("Part2: {}", max.1 - min.1);
            }
        }
    }
}

fn update_counts(old_counts: &HashMap<(char, char), i64>, operations: &HashMap<(char, char), char>) -> HashMap<(char, char), i64> {
    let mut new_counts = HashMap::new();
    old_counts.iter()
        .map(|((p0, p1), count)| {
            ((p0, operations.get(&(*p0, *p1)).unwrap(), p1), count)
        })
        .for_each(|((p0, to, p1), count)| {
            *new_counts.entry((*p0, *to)).or_insert(0) += *count;
            *new_counts.entry((*to, *p1)).or_insert(0) += *count;
        });
    new_counts
}
