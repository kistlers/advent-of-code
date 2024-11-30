use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry::Vacant;
use elf::measure;
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;

// Code for the problem https://adventofcode.com/2022/day/23
const YEAR: u32 = 2022;
const DAY: u32 = 23;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "53616c7465645f5f235e9a21eb1ad1558a877268d5a6533d8a3c56c539f0886b51e27e5ccf230b8815cb5ca121c1f3173a16dd022c3803eefcd5e6fb5f077dae");
    let positions = input_lines.iter().enumerate()
        .flat_map(|(i, input_line)| input_line.chars().enumerate().filter(|(_, c)| *c == '#').map(|(j, _)| (i as isize, j as isize)).collect_vec())
        .collect::<HashSet<_>>();

    let (part1, part2) = measure!({simulate_rounds(positions)});
    println!("part1: {part1}");
    println!("part2: {part2}");
}

fn simulate_rounds(mut positions: HashSet<(isize, isize)>) -> (isize, isize) {
    let mut directions = ['N', 'S', 'W', 'E'];
    directions.rotate_right(1);
    let mut round = 0;
    let mut after_10 = 0;
    loop {
        let mut proposed_positions: HashMap<(isize, isize), isize> = HashMap::new();
        directions.rotate_left(1);
        let mut all_staying = true;
        let elves = positions.iter()
            .map(|(i, j)| {
                let proposed = if needs_to_not_move(*i, *j, &positions) {
                    (*i, *j)
                } else {
                    all_staying = false;
                    propose_position(*i, *j, &positions, &directions)
                };
                if let Vacant(entry) = proposed_positions.entry(proposed) {
                    entry.insert(1);
                } else {
                    proposed_positions.insert(proposed, proposed_positions[&proposed] + 1);
                }
                ((*i, *j), proposed)
            })
            .collect::<HashSet<_>>();
        if all_staying {
            if round < 10 {
                after_10 = calc_after_10(&positions);
            }
            break;
        }
        positions = elves.iter()
            .map(|(curr, proposed)| if proposed_positions[proposed] == 1 { *proposed } else { *curr })
            .collect::<HashSet<_>>();

        round += 1;
        if round == 10 {
            after_10 = calc_after_10(&positions);
        }
    }

    (after_10, round + 1)
}

fn calc_after_10(positions: &HashSet<(isize, isize)>) -> isize {
    let (i_min, i_max) = get_min_max(positions, |(i, _)| i);
    let (j_min, j_max) = get_min_max(positions, |(_, j)| j);
    (i_max - i_min + 1) * (j_max - j_min + 1) - positions.len() as isize
}

fn needs_to_not_move(i: isize, j: isize, positions: &HashSet<(isize, isize)>) -> bool {
    !positions.contains(&(i - 1, j - 1))
        && !positions.contains(&(i - 1, j))
        && !positions.contains(&(i - 1, j + 1))
        && !positions.contains(&(i, j + 1))
        && !positions.contains(&(i + 1, j + 1))
        && !positions.contains(&(i + 1, j))
        && !positions.contains(&(i + 1, j - 1))
        && !positions.contains(&(i, j - 1))
}

fn get_min_max(positions: &HashSet<(isize, isize)>, get_val: fn(&(isize, isize)) -> &isize) -> (isize, isize) {
    if let MinMax(min, max) = positions.iter().map(get_val).minmax() { (*min, *max) } else { panic!() }
}

fn propose_position(i: isize, j: isize, positions: &HashSet<(isize, isize)>, directions: &[char; 4]) -> (isize, isize) {
    for d in directions {
        match d {
            'N' => if !positions.contains(&(i - 1, j - 1)) && !positions.contains(&(i - 1, j)) && !positions.contains(&(i - 1, j + 1)) { return (i - 1, j); },
            'S' => if !positions.contains(&(i + 1, j - 1)) && !positions.contains(&(i + 1, j)) && !positions.contains(&(i + 1, j + 1)) { return (i + 1, j); },
            'E' => if !positions.contains(&(i - 1, j + 1)) && !positions.contains(&(i, j + 1)) && !positions.contains(&(i + 1, j + 1)) { return (i, j + 1); },
            'W' => if !positions.contains(&(i - 1, j - 1)) && !positions.contains(&(i, j - 1)) && !positions.contains(&(i + 1, j - 1)) { return (i, j - 1); },
            _ => panic!()
        }
    }
    (i, j)
}
