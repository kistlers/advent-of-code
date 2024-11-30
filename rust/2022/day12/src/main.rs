use std::collections::VecDeque;
use itertools::Itertools;

// Code for the problem https://adventofcode.com/2022/day/12
const YEAR: u32 = 2022;
const DAY: u32 = 12;

const D_VECTORS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "<>");
    let grid = input_lines.iter()
        .map(|input_line| input_line.chars().collect_vec())
        .collect_vec();

    let height = grid.len();
    let width = grid[0].len();
    let mut e: (i32, i32, i32) = (0, 0, 0);
    for i in 0..height {
        for j in 0..width {
            if grid[i][j] == 'E' {
                e = (i as i32, j as i32, 0);
                break;
            }
        }
    }

    println!("part1: {}", bfs(&grid, e, &['S'], height, width));
    println!("part2: {}", bfs(&grid, e, &['a', 'S'], height, width));
}

fn bfs(grid: &[Vec<char>], e: (i32, i32, i32), targets: &[char], height: usize, width: usize) -> usize {
    let mut distance = vec![vec![1_000_000; width]; height];
    let mut queue = VecDeque::from([e]);
    while !queue.is_empty() {
        let (i, j, steps) = queue.pop_front().unwrap();
        if targets.contains(&grid[i as usize][j as usize]) {
            return steps as usize;
        }
        if distance[i as usize][j as usize] <= steps {
            continue;
        }
        distance[i as usize][j as usize] = steps;

        let curr_char = if grid[i as usize][j as usize] == 'E' { 'z' } else { grid[i as usize][j as usize] };

        let new_edges = D_VECTORS.iter()
            .map(|(di, dj)| (i + di, j + dj, steps + 1))
            .filter(|(new_i, new_j, _)| {
                if *new_i < 0 || *new_i >= height as i32 || *new_j < 0 || *new_j >= width as i32 {
                    return false;
                }
                let new_char = if grid[*new_i as usize][*new_j as usize] == 'S' { 'a' } else { grid[*new_i as usize][*new_j as usize] };
                if (new_char as u32) < (curr_char as u32 - 1) {
                    return false;
                }
                true
            })
            .collect_vec();
        queue.extend(new_edges);
    }
    1_000_000
}
