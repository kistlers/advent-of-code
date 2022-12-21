use elf::SplitExtension;
use itertools::Itertools;

// Code for the problem https://adventofcode.com/2022/day/14
const YEAR: u32 = 2022;
const DAY: u32 = 14;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "<>");
    let paths = input_lines.iter()
        .map(|input_line| input_line
            .split(" -> ")
            .map(|point| point.split_once_parse::<usize, usize>(",").unwrap())
            .collect_vec()
        ).collect_vec();

    let y_bottom = *paths.iter().flatten().map(|(_, y)| y).max().unwrap() + 2;

    // 0 empty, 1 wall, 2 sand
    let mut grid_pristine = vec![vec![0_usize; 1001]; y_bottom + 10];
    for path in paths {
        for step in path.windows(2) {
            let (x_from, y_from, x_to, y_to): (usize, usize, usize, usize)
                = if let Some(((x_from, y_from), (x_to, y_to))) = step.iter().collect_tuple() { (*x_from, *y_from, *x_to, *y_to) } else { panic!() };
            // println!("{},{} -> {},{}", x_from, y_from, x_to, y_to);
            if x_from == x_to {
                for y in y_from.min(y_to)..=y_to.max(y_from) {
                    grid_pristine[y][x_from] = 1;
                }
            } else {
                for x in x_from.min(x_to)..=x_to.max(x_from) {
                    grid_pristine[y_from][x] = 1;
                }
            }
        }
    }
    // floor, only necessary for part2,  can also stay for part1
    for x in 0..=1000 {
        grid_pristine[y_bottom][x] = 1;
    }

    let mut grid = grid_pristine.clone();
    let part1 = loop {
        // print_grid(&grid);
        let went_into_abyss = next_sand(&mut grid, false, y_bottom);
        if went_into_abyss {
            break grid.iter().flatten().filter(|g| **g == 2).count();
        }
    };
    println!("part1: {part1}");

    let mut grid = grid_pristine.clone();
    let part2 = loop {
        // print_grid(&grid, y_max + 10);
        let blocked = next_sand(&mut grid, true, 0);
        if blocked {
            break grid.iter().flatten().filter(|g| **g == 2).count();
        }
    };
    println!("part2: {part2}");
}

fn next_sand(grid: &mut [Vec<usize>], has_floor: bool, y_abyss: usize) -> bool {
    let mut sx: usize = 500;
    let mut sy: usize = 0;
    loop {
        if !has_floor && sy + 1 == y_abyss {
            return true;
        }
        if grid[sy + 1][sx] == 0 {
            sy += 1;
            continue;
        }
        if grid[sy + 1][sx - 1] == 0 {
            sy += 1;
            sx -= 1;
            continue;
        }
        if grid[sy + 1][sx + 1] == 0 {
            sy += 1;
            sx += 1;
            continue;
        }
        if has_floor && grid[sy][sx] != 0 {
            return true;
        }
        grid[sy][sx] = 2;
        return false;
    }
}

#[allow(dead_code)]
fn print_grid(grid: &[Vec<usize>], y: usize) {
    for down in 0..y {
        for right in (500 - y)..=(500 + y) {
            print!("{}", match grid[down][right] {
                0 => '.',
                1 => '#',
                2 => 'o',
                _ => panic!()
            });
        }
        println!();
    }
    println!();
}