// Code for the problem https://adventofcode.com/2021/day/13
const YEAR: u32 = 2021;
const DAY: u32 = 13;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "53616c7465645f5fc3caba04b37cfdc549d664f17097aa5457af16a74fbc99744bebc15d2bc22c9784c1224456ac0746");
    let mut coords: Vec<(usize, usize)> = vec![];
    let mut folds: Vec<(char, usize)> = vec![];
    for input_line in input_lines.iter().filter(|l| !l.trim_start().is_empty()) {
        let chars = input_line.chars().collect::<Vec<char>>();
        if chars.first().unwrap().is_numeric() {
            coords.push(input_line
                .split_once(',')
                .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                .unwrap());
        } else if input_line.starts_with("fold along ") {
            match input_line.trim_start_matches("fold along ").chars().next() {
                Some('y') => folds.push(('y', input_line.trim_start_matches("fold along y=").parse::<usize>().unwrap())),
                Some('x') => folds.push(('x', input_line.trim_start_matches("fold along x=").parse::<usize>().unwrap())),
                _ => {}
            }
            if input_line.starts_with("fold along y=") {}
        }
    }
    let y_size = folds.iter().filter(|(dir, _)| *dir == 'y').map(|(_, val)| val).max().unwrap()*2 + 1;
    let x_size = folds.iter().filter(|(dir, _)| *dir == 'x').map(|(_, val)| val).max().unwrap()*2 + 1;
    let mut grid = vec![vec![false; x_size]; y_size];
    for (x, y) in coords {
        grid[y][x] = true;
    }

    let mut part1_already_printed = false;
    for (dir, val) in folds {
        match dir {
            'x' => {
                let mut right = vec![];
                get_x_from_reversed(&mut right, &grid, val + 1);
                grid = get_x_to(&grid, val);
                assert_eq!(right.len(), grid.len());
                assert_eq!(right[0].len(), grid[0].len());
                overlap(&mut grid, &right);
            }
            'y' => {
                let mut lower = grid[(val + 1)..grid.len()].to_vec();
                lower.reverse();
                grid = grid[0..val].to_vec();
                overlap(&mut grid, &lower);
            }
            _ => panic!()
        }
        if !part1_already_printed {
            part1_already_printed = true;
            println!("Part1: {}", grid.iter().flatten().filter(|x| **x).count());
        }
    }

    println!();
    for row in &grid {
        for b in row {
            print!("{}", if *b { '#' } else { ' ' });
        }
        println!();
    }
}

fn get_x_to(grid: &[Vec<bool>], x_to: usize) -> Vec<Vec<bool>> {
    let mut left: Vec<Vec<bool>> = vec![];
    for row in grid {
        left.push(row[0..x_to].to_vec());
    }
    left
}

fn get_x_from_reversed(into: &mut Vec<Vec<bool>>, grid: &[Vec<bool>], x_from: usize) {
    for row in grid {
        let mut new_row = row[x_from..row.len()].to_vec();
        assert_eq!(row.len(), (new_row.len() * 2) + 1);
        new_row.reverse();
        into.push(new_row);
    }
}

fn overlap(grid: &mut Vec<Vec<bool>>, overlap: &[Vec<bool>]) {
    for y in 0..overlap.len() {
        for x in 0..overlap[y].len() {
            grid[y][x] |= overlap[y][x];
        }
    }
}
