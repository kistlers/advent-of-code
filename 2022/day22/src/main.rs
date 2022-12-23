use elf::measure;
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use lazy_static::lazy_static;
use regex::Regex;

// Code for the problem https://adventofcode.com/2022/day/22
const YEAR: u32 = 2022;
const DAY: u32 = 22;

fn main() {
    let input_lines: Vec<String> = elf::get_input_no_trim(YEAR, DAY, "53616c7465645f5f235e9a21eb1ad1558a877268d5a6533d8a3c56c539f0886b51e27e5ccf230b8815cb5ca121c1f3173a16dd022c3803eefcd5e6fb5f077dae");

    let board_width = input_lines.iter()
        .take_while(|input_line| !input_line.is_empty())
        .map(|input_line| input_line.len())
        .max().unwrap();
    // 0 off the map, 1 walkable, 2 wall
    let board = input_lines.iter()
        .take_while(|input_line| !input_line.is_empty())
        .map(|input_line| {
            let mut tiles = input_line.chars().collect_vec();
            if tiles.len() < board_width {
                tiles.extend([' '].repeat(board_width - tiles.len()))
            }
            tiles
        })
        .collect_vec();
    let first_last_row = &board.iter()
        .map(|row|
            if let MinMax(first, last) = row.iter().positions(|&b| b == '.' || b == '#').minmax() { (first, last) } else { panic!() }
        ).collect_vec();
    let first_last_col = transpose(board.clone()).iter()
        .map(|column|
            if let MinMax(first, last) = column.iter().positions(|&b| b == '.' || b == '#').minmax() { (first, last) } else { panic!() }
        ).collect_vec();

    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"(\d+)(L|R)").unwrap();
    }

    let path = REGEX.captures_iter(input_lines.last().unwrap())
        .map(|captures| (captures[1].parse::<usize>().unwrap(), captures[2].chars().take(1).collect_vec()[0]))
        .collect_vec();

    let cube_side_len = board_width / 3;

    println!("{}", board_width);
    // println!("{:?}", board);
    // println!("{:?}", first_last_row);
    // println!("{:?}", first_last_col);
    // println!("{:?}", path);
    let (part1, part2) = measure!({
        let part1 = find_final_facing_flat(&board, first_last_row, &first_last_col, &path);
        let part2 = find_final_facing_cube(&board, first_last_row, &first_last_col, &path, cube_side_len);
        (part1, part2)
    });
    println!("part1: {part1}");
    println!("part2: {part2}");
}

fn find_final_facing_cube(board: &[Vec<char>], first_last_row: &[(usize, usize)], first_last_col: &[(usize, usize)], path: &[(usize, char)], cube_side_len: usize) -> usize {
    let mut facing = 0;
    let mut col = first_last_row[0].0;
    let mut row = 0;

    for (amount, dir) in path.iter() {
        for _ in 0..*amount {
            // println!("at ({row}, {col}) facing {facing}");
            let (new_col, new_row) = wrap_cube(next_pos_flat(facing, col, row, board, first_last_row, first_last_col), cube_side_len);
            if board[new_row][new_col] == '#' {
                break;
            }
            col = new_col;
            row = new_row;
        }
        facing = next_facing(facing, dir);
    }

    1000 * (row + 1) + 4 * (col + 1) + facing
}

fn wrap_cube(col: isize, row: isize, cube_side_len: usize) -> (usize, usize) {
    if row == -1  && col as usize > 2*cube_side_len && col as usize <= 3*cube_side_len{
        // 1st face, at top
    } else if row as usize == cube_side_len && col as usize{
        // 2nd face, at top
        let row = row as usize;
    } else if row as usize == 2 * cube_side_len {
        let row = row as usize;
    } else if row as usize == 2 * cube_side_len + 1 {
        let row = row as usize;
    } else if row == 3 * cube_side_len + 1 {
        let row = row as usize;
    } else {
        panic!()
    }
}

fn next_pos_cube(facing: usize, col: usize, row: usize, board: &[Vec<char>], first_last_row: &[(usize, usize)], first_last_col: &[(usize, usize)], cube_side_len: usize) -> (usize, usize) {
    match facing {
        0 => { if col == first_last_row[row].1 || board[row][col + 1] == ' ' { wrap_cube(col + 1, row, cube_side_len) } else { (col + 1, row) } }
        1 => { if row == first_last_col[col].1 || board[row + 1][col] == ' ' { wrap_cube(col, row + 1, cube_side_len) } else { (col, row + 1) } }
        2 => { if col == first_last_row[row].0 || board[row][col - 1] == ' ' { wrap_cube(col as isize - 1, row, cube_side_len) } else { (col - 1, row) } }
        3 => { if row == first_last_col[col].0 || board[row - 1][col] == ' ' { wrap_cube(col, row as isize - 1, cube_side_len) } else { (col, row - 1) } }
        _ => panic!()
    }
}

fn find_final_facing_flat(board: &[Vec<char>], first_last_row: &[(usize, usize)], first_last_col: &[(usize, usize)], path: &[(usize, char)]) -> usize {
    let mut facing = 0;
    let mut col = first_last_row[0].0;
    let mut row = 0;

    for (amount, dir) in path.iter() {
        for _ in 0..*amount {
            // println!("at ({row}, {col}) facing {facing}");
            let (new_col, new_row) = next_pos_flat(facing, col, row, board, first_last_row, first_last_col);
            if board[new_row][new_col] == '#' {
                break;
            }
            col = new_col;
            row = new_row;
        }
        facing = next_facing(facing, dir);
    }

    1000 * (row + 1) + 4 * (col + 1) + facing
}

fn next_pos_flat(facing: usize, col: usize, row: usize, board: &[Vec<char>], first_last_row: &[(usize, usize)], first_last_col: &[(usize, usize)]) -> (usize, usize) {
    match facing {
        0 => { if col == first_last_row[row].1 || board[row][col + 1] == ' ' { (first_last_row[row].0, row) } else { (col + 1, row) } }
        1 => { if row == first_last_col[col].1 || board[row + 1][col] == ' ' { (col, first_last_col[col].0) } else { (col, row + 1) } }
        2 => { if col == first_last_row[row].0 || board[row][col - 1] == ' ' { (first_last_row[row].1, row) } else { (col - 1, row) } }
        3 => { if row == first_last_col[col].0 || board[row - 1][col] == ' ' { (col, first_last_col[col].1) } else { (col, row - 1) } }
        _ => panic!()
    }
}

fn next_facing(facing: usize, dir: &char) -> usize {
    match (facing, dir) {
        (0, 'L') => 3,
        (1, 'L') => 0,
        (2, 'L') => 1,
        (3, 'L') => 2,
        (0, 'R') => 1,
        (1, 'R') => 2,
        (2, 'R') => 3,
        (3, 'R') => 0,
        _ => panic!()
    }
}

fn transpose<T>(original: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!original.is_empty());
    let mut transposed = (0..original[0].len()).map(|_| vec![]).collect::<Vec<_>>();

    for original_row in original {
        for (item, transposed_row) in original_row.into_iter().zip(&mut transposed) {
            transposed_row.push(item);
        }
    }

    transposed
}
