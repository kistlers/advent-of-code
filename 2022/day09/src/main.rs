extern crate core;

use std::collections::HashSet;
use elf::SplitExtension;
use itertools::Itertools;

// Code for the problem https://adventofcode.com/2022/day/09
const YEAR: u32 = 2022;
const DAY: u32 = 09;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "<>");
    let moves = input_lines.iter().map(|input_line| input_line.split_once_parse::<char, i32>(" ").unwrap()).collect_vec();

    let (part1, part2) = count_visited_2(&moves, 9);
    println!("part1: {part1}");
    println!("part2: {part2}");
}

fn count_visited_2(moves: &[(char, i32)], tails: usize) -> (usize, usize) {
    let mut visited = vec![HashSet::<(i32, i32)>::new(); tails];
    let mut txs = vec![0; tails + 1];
    let mut tys = vec![0; tails + 1];

    for (dir, amount) in moves {
        for _ in 0..*amount {
            move_head(*dir, &mut txs[0], &mut tys[0]);
            for t in 0..tails {
                move_tail(
                    &mut txs[t],
                    &mut tys[t],
                    &mut txs[t + 1],
                    &mut tys[t + 1],
                );
                visited[t].insert((txs[t + 1], tys[t + 1]));
            }
        }
    }

    (visited[0].len(), visited[tails - 1].len())
}

fn move_head(dir: char, hx: *mut i32, hy: *mut i32) {
    let (dx, dy) = match dir {
        'R' => (1, 0),
        'L' => (-1, 0),
        'U' => (0, 1),
        'D' => (0, -1),
        _ => panic!()
    };
    unsafe {
        *hx += dx;
        *hy += dy;
    }
}

fn move_tail(hx: *mut i32, hy: *mut i32, tx: *mut i32, ty: *mut i32) {
    unsafe {
        if (*hx - *tx).abs() > 1 || (*hy - *ty).abs() > 1 {
            *tx += match *hx - *tx {
                i if i > 0 => 1,
                i if i < 0 => -1,
                _ => 0
            };
            *ty += match *hy - *ty {
                i if i > 0 => 1,
                i if i < 0 => -1,
                _ => 0
            };
        }
    }
}
