use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use grouping_by::GroupingBy;

// Code for the problem https://adventofcode.com/2021/day/8
const YEAR: u32 = 2021;
const DAY: u32 = 8;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "53616c7465645f5fc3caba04b37cfdc549d664f17097aa5457af16a74fbc99744bebc15d2bc22c9784c1224456ac0746");
    let output_lines = input_lines.iter().flat_map(|line| line.split(" | ").collect::<Vec<_>>()[1].split(' ')).collect::<Vec<_>>();
    // lengths 2, 4, 3, 7 correspond to digits 1, 4, 7, 8
    let count1478 = output_lines.iter().filter(|p| [2, 4, 3, 7].contains(&p.len())).count();
    println!("Part1: {}", count1478);

    let sum = input_lines.iter().map(|input_line| {
        let (signal_patterns, outputs) = input_line
            .split_once(" | ")
            .map(|(s, o)| (s.split_whitespace().collect::<Vec<_>>(), o.split_whitespace().collect::<Vec<_>>()))
            .unwrap();
        let grouped_by_len = signal_patterns.iter()
            .map(|s| HashSet::from_iter(s.chars()))
            .grouping_by(|p| p.len());

        let digit_mappings = get_digit_map(grouped_by_len);

        outputs
            .iter()
            .map(|output| convert_to_digit(output, &digit_mappings))
            .collect::<String>().parse::<i32>()
            .unwrap()
    }).sum::<i32>();
    println!("Part2: {}", sum);
}

fn convert_to_digit(output: &&str, digit_mappings: &HashMap<char, usize>) -> char {
    let mut segments = output
        .chars()
        .map(|o| digit_mappings[&o])
        .collect::<Vec<_>>();
    segments.sort_unstable();
    match segments[..] {
        [0, 1, 2, 4, 5, 6] => '0',
        [2, 5] => '1',
        [0, 2, 3, 4, 6] => '2',
        [0, 2, 3, 5, 6] => '3',
        [ 1, 2, 3, 5] => '4',
        [0, 1, 3, 5, 6] => '5',
        [0, 1, 3, 4, 5, 6] => '6',
        [0, 2, 5] => '7',
        [0, 1, 2, 3, 4, 5, 6] => '8',
        [0, 1, 2, 3, 5, 6] => '9',
        _ => panic!()
    }
}

fn get_digit_map(grouped_by_len: HashMap<usize, Vec<HashSet<char>>>) -> HashMap<char, usize> {
    let all_chars = HashSet::<char>::from_iter("abcdefg".chars());
    let mut digit_map = HashMap::<char, usize>::new();

    // 0 is in 7 (len 3) but not in 1 (len 2)
    digit_map.insert(
        *grouped_by_len[&3][0]
            .difference(&grouped_by_len[&2][0])
            .into_iter()
            .collect::<Vec<_>>()[0],
        0);

    // 5 is in 1 (len 2) and in all of len 6
    let five = grouped_by_len[&2][0]
        .iter()
        .filter(|b| grouped_by_len[&6].iter().all(|set| set.contains(*b)))
        .copied()
        .collect::<Vec<_>>()[0];
    digit_map.insert(five, 5);

    // 2 is the other segment of 1
    digit_map.insert(
        grouped_by_len[&2][0]
            .difference(&HashSet::from([five]))
            .copied()
            .collect::<Vec<_>>()[0], 2);

    let segments_of_four_diff_one = grouped_by_len[&4][0]
        .difference(&grouped_by_len[&2][0])
        .copied()
        .collect::<HashSet<_>>();

    // 1 is in segments_of_four_diff_one and in all of len 6
    let one = segments_of_four_diff_one
        .iter()
        .filter(|b| grouped_by_len[&6].iter().all(|set| set.contains(*b)))
        .copied()
        .collect::<Vec<_>>()[0];
    digit_map.insert(one, 1);

    // 3 is the other segment of segments_of_four_diff_one
    digit_map.insert(
        segments_of_four_diff_one
            .difference(&HashSet::from([one]))
            .copied()
            .collect::<Vec<_>>()[0], 3);

    let remaining2 = all_chars.iter().filter(|c| !digit_map.contains_key(c)).copied().collect::<HashSet<_>>();

    // 6 is the segment of remaining2 that is in all of len 5
    let six = remaining2
        .iter()
        .filter(|b| grouped_by_len[&5].iter().all(|set| set.contains(*b)))
        .copied()
        .collect::<Vec<_>>()[0];
    digit_map.insert(six, 6);

    // 4 is the other segment of remaining2
    digit_map.insert(
        remaining2
            .difference(&HashSet::from([six]))
            .copied()
            .collect::<Vec<_>>()[0], 4);

    digit_map
}
