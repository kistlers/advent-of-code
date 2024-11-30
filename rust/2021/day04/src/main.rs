// Code for the problem https://adventofcode.com/2021/day/4
const YEAR: u32 = 2021;
const DAY: u32 = 4;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "53616c7465645f5fc3caba04b37cfdc549d664f17097aa5457af16a74fbc99744bebc15d2bc22c9784c1224456ac0746");
    let mut lines_iter = input_lines.iter();
    let drawn = lines_iter.next().unwrap()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    lines_iter.next();

    let mut board: Vec<i32> = vec![];
    let mut boards: Vec<Vec<i32>> = vec![];
    for input_line in lines_iter {
        if !input_line.is_empty() {
            // println!("{}", input_line);
            board.append(&mut input_line.split_whitespace().map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>());
        } else {
            boards.push(board);
            board = vec![];
        }
    }
    boards.push(board);

    let mut num_boards = boards.len();
    let mut score_rows: Vec<Vec<i32>> = vec![vec![0; 5]; num_boards];
    let mut score_cols: Vec<Vec<i32>> = vec![vec![0; 5]; num_boards];

    let mut num_boards_left = num_boards;
    let mut board_already_won = vec![false; num_boards];
    let mut winner_found = false;
    let mut winning_board: usize = 0;
    let mut winning_draw: i32 = 0;

    for draw in drawn {
        for board_index in 0..num_boards {
            if !board_already_won[board_index] {
                let board = boards.get_mut(board_index).unwrap();
                for i in 0..board.len() {
                    if board[i] == draw {
                        board[i] = -1;
                        score_cols[board_index][i % 5] += 1;
                        score_rows[board_index][i / 5] += 1;
                        if score_cols[board_index][i % 5] == 5 || score_rows[board_index][i / 5] == 5 {
                            winning_board = board_index;
                            winning_draw = draw;
                            board_already_won[board_index] = true;
                            num_boards_left -= 1;
                            if num_boards_left == 0 {
                                winner_found = true;
                                break;
                            }
                        }
                    }
                }
                if winner_found {
                    break;
                }
            }
        }
        if winner_found {
            break;
        }
    }
    println!("Part1: {}", boards.get(winning_board).unwrap().iter().fold(0, |sum, x| if *x >= 0 { sum + *x } else { sum }) * winning_draw);
}
