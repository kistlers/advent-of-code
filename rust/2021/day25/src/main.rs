// Code for the problem https://adventofcode.com/2021/day/25
const YEAR: u32 = 2021;
const DAY: u32 = 25;

const EAST: char = '>';
const SOUTH: char = 'v';
const DOT: char = '.';

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "53616c7465645f5fc3caba04b37cfdc549d664f17097aa5457af16a74fbc99744bebc15d2bc22c9784c1224456ac0746");
    let mut grid = input_lines.iter().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut step = 0;
    loop {
        let (did_move_east, new_grid) = move_east(grid, rows, cols);
        grid = new_grid;
        let (did_move_south, new_grid) = move_south(grid, rows, cols);
        grid = new_grid;
        step += 1;
        if !did_move_east && !did_move_south {
            break;
        }
    }
    println!("Part1: {}", step);
}

fn move_east(grid: Vec<Vec<char>>, rows: usize, cols: usize) -> (bool, Vec<Vec<char>>) {
    let mut new_grid = vec![vec!['.'; cols]; rows];
    let mut did_move = false;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == EAST && grid[r][(c + 1) % cols] == DOT {
                new_grid[r][(c + 1) % cols] = EAST;
                did_move = true;
            } else if new_grid[r][c] == DOT {
                new_grid[r][c] = grid[r][c];
            }
        }
    }

    (did_move, new_grid)
}

fn move_south(grid: Vec<Vec<char>>, rows: usize, cols: usize) -> (bool, Vec<Vec<char>>) {
    let mut new_grid = vec![vec!['.'; cols]; rows];
    let mut did_move = false;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == SOUTH && grid[(r + 1) % rows][c] == DOT {
                new_grid[(r + 1) % rows][c] = SOUTH;
                did_move = true;
            } else if new_grid[r][c] == DOT {
                new_grid[r][c] = grid[r][c];
            }
        }
    }

    (did_move, new_grid)
}