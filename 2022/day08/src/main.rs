use itertools::Itertools;

// Code for the problem https://adventofcode.com/2022/day/08
const YEAR: u32 = 2022;
const DAY: u32 = 08;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "<>");
    let grid = input_lines.iter()
        .map(|input_line| input_line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect_vec())
        .collect_vec();
    let height = grid.len();
    let width = grid[0].len();

    let mut part1 = 2 * (height + width - 2);
    let mut part2 = 1;
    for i in 1..height - 1 {
        for j in 1..width - 1 {
            let tree = grid[i][j];

            let vec_top = (0..i).rev().map(|ii| grid[ii][j]).collect_vec();
            let vec_bottom = (i + 1..height).map(|ii| grid[ii][j]).collect_vec();
            let vec_left = (0..j).rev().map(|jj| grid[i][jj]).collect_vec();
            let vec_right = (j + 1..width).map(|jj| grid[i][jj]).collect_vec();

            let visible = visible(tree, &vec_top) || visible(tree, &vec_bottom) || visible(tree, &vec_left) || visible(tree, &vec_right);
            if visible {
                part1 += 1;
            }

            let score = score(tree, &vec_top) * score(tree, &vec_bottom) * score(tree, &vec_left) * score(tree, &vec_right);
            if score > part2 {
                part2 = score;
            }
        }
    }
    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

fn visible(tree: usize, heights: &[usize]) -> bool {
    heights.iter().all(|t| *t < tree)
}

fn score(tree: usize, heights: &[usize]) -> usize {
    for (score, t) in heights.iter().enumerate() {
        if *t >= tree {
            return score + 1;
        }
    }
    heights.len()
}
