use std::collections::HashMap;

// Code for the problem https://adventofcode.com/2021/day/6
const YEAR: u32 = 2021;
const DAY: u32 = 6;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "53616c7465645f5fc3caba04b37cfdc549d664f17097aa5457af16a74fbc99744bebc15d2bc22c9784c1224456ac0746");
    let init_state = input_lines.get(0).unwrap().split(",").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();

    let mut to_spawn = (0..=8).map(|d| (d, 0)).collect::<HashMap<i64, i64>>();
    for s in &init_state {
        to_spawn.insert(*s, to_spawn[s] + 1);
    }

    for i in 1..=256 {
        let new_counts: HashMap<i64, i64> = (0..=8).map(|d| match d {
            8 => (d, to_spawn[&0]),
            6 => (d, to_spawn[&7] + to_spawn[&0]),
            _ => (d, to_spawn[&(d + 1)])
        }).collect();
        to_spawn = new_counts;
        if [80, 256].contains(&i) {
            println!("after day {}, {:?}", i, to_spawn.values().sum::<i64>());
        }
    }
}