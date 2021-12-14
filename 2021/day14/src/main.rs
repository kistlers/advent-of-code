use std::collections::{BTreeMap, HashMap};
use std::iter::once;
use elf::SplitExtension;

// Code for the problem https://adventofcode.com/2021/day/14
const YEAR: u32 = 2021;
const DAY: u32 = 14;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "53616c7465645f5fc3caba04b37cfdc549d664f17097aa5457af16a74fbc99744bebc15d2bc22c9784c1224456ac0746");
    let mut iter = input_lines.iter();
    let mut sequence = input_lines[0].chars().collect::<Vec<_>>();
    // println!("{}", sequence);
    iter.next();
    let operations = input_lines[2..]
        .iter()
        .map(|line| line.split_once_parse_value::<char>(" -> ").unwrap())
        .map(|(from, to)| (from.chars().collect::<Vec<_>>(), to))
        .map(|(mut from, to)| (from.clone(), intersplice_in_middle(&mut from, to)))
        .collect::<HashMap<Vec<char>, Vec<char>>>();
    // for (from, to) in &operations {
    //     println!("{} to {}", from, to);
    // }
    for step in 1..=40 {
        // let mut new_sequence = sequence.clone();
        let mapped_windows = sequence
            .windows(2)
            .map(|s| operations[s].clone())
            .collect::<Vec<_>>();
        let new_sequence = mapped_windows
            .iter()
            .map(|s| [s[0], s[1]])
            .flatten()
            .chain(once(*mapped_windows.last().unwrap().last().unwrap()))
            .collect::<Vec<_>>();
        // println!("before: {}", String::from_iter(&sequence));
        // println!("after:  {}", String::from_iter(&new_sequence));
        sequence = new_sequence;
        if step == 10 || step == 40 {
            let mut counts = BTreeMap::new();
            for c in sequence.iter() {
                *counts.entry(c).or_insert(0) += 1;
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
        println!("step {}", step);
    }

}

fn intersplice_in_middle(into: &mut Vec<char>, c: char) -> Vec<char> {
    into.insert(1, c);
    into.to_owned()
}
