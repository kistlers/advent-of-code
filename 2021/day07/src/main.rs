// Code for the problem https://adventofcode.com/2021/day/7
const YEAR: u32 = 2021;
const DAY: u32 = 7;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "53616c7465645f5fc3caba04b37cfdc549d664f17097aa5457af16a74fbc99744bebc15d2bc22c9784c1224456ac0746");
    let mut positions = input_lines[0].split(",").map(|p| p.parse::<i32>().unwrap()).collect::<Vec<_>>();
    positions.sort();
    let median = positions[(positions.len() as f64 / 2.0).ceil() as usize];
    let sum = positions.iter().map(|p| (p - median).abs()).sum::<i32>();
    println!("Part1: {}", sum);

    let mut min_sum = i32::MAX;
    for align in &positions {
        let sum = positions.iter().map(|p| (p - *align).abs()).map(|m| m * (m + 1) / 2).sum::<i32>();
        min_sum = if sum < min_sum { sum } else { min_sum };
    }
    println!("Part2: {}", min_sum);
}
