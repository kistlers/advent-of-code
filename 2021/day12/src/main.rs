use std::collections::HashMap;

// Code for the problem https://adventofcode.com/2021/day/12
const YEAR: u32 = 2021;
const DAY: u32 = 12;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "53616c7465645f5fc3caba04b37cfdc549d664f17097aa5457af16a74fbc99744bebc15d2bc22c9784c1224456ac0746");

    let mut cave_system = HashMap::<String, Vec<String>>::new();
    input_lines.iter().map(|line| line.split_once('-').map(|(f, t)| (f.to_string(), t.to_string())).unwrap()).for_each(|(from, to)| {
        cave_system.entry(from.clone()).or_insert_with(Vec::new).push(to.clone());
        cave_system.entry(to).or_insert_with(Vec::new).push(from);
    });

    let mut paths1: Vec<Vec<String>> = vec![];
    find_paths(&cave_system, &mut paths1, vec!["start".to_string()], true);
    println!("Part1: {}", paths1.len());

    let mut paths2: Vec<Vec<String>> = vec![];
    find_paths(&cave_system, &mut paths2, vec!["start".to_string()], false);
    println!("Part2: {}", paths2.len());
}

fn find_paths(cave_system: &HashMap<String, Vec<String>>, paths: &mut Vec<Vec<String>>, current_path: Vec<String>, single_small_already_visited: bool) {
    for next_node in &cave_system[current_path.last().unwrap()] {
        let path_to_next = current_path.iter().chain(&[next_node.to_owned()]).cloned().collect::<Vec<_>>();
        let next_small_already_visited = current_path.iter().filter(|n| **n == n.to_lowercase()).any(|x| *x == *next_node);

        if next_node == "end" {
            paths.push(path_to_next);
        } else if !(single_small_already_visited && next_small_already_visited) && next_node != "start" {
            find_paths(cave_system, paths, path_to_next, single_small_already_visited || next_small_already_visited);
        }
    }
}