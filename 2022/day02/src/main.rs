use elf::SplitExtension;

// Code for the problem https://adventofcode.com/2022/day/02
const YEAR: u32 = 2022;
const DAY: u32 = 02;

// Part 1
// A, X Rock
// B, Y Paper
// C, Z Scissors

// Part 2
// X lose
// Y draw
// Z win

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "53616c7465645f5f235e9a21eb1ad1558a877268d5a6533d8a3c56c539f0886b51e27e5ccf230b8815cb5ca121c1f3173a16dd022c3803eefcd5e6fb5f077dae");
    let part1 = &input_lines.iter().map(|input_line| {
        let (opponent, me): (char, char) = input_line.split_once_parse(" ").unwrap();
        score1(opponent, me)
    }).sum::<u32>();
    println!("part1: {}", part1);

    let part2 = &input_lines.iter().map(|input_line| {
        let (opponent, what_to_do): (char, char) = input_line.split_once_parse(" ").unwrap();
        score2(opponent, what_to_do)
    }).sum::<u32>();
    println!("part2: {}", part2);
}

fn win(opponent: char, me: char) -> bool {
    me == 'X' && opponent == 'C' || me == 'Y' && opponent == 'A' || me == 'Z' && opponent == 'B'
}

fn draw(opponent: char, me: char) -> bool {
    me == 'X' && opponent == 'A' || me == 'Y' && opponent == 'B' || me == 'Z' && opponent == 'C'
}

fn score_chosen(chosen: char) -> u32 {
    if chosen == 'X' || chosen == 'A' { 1 } else if chosen == 'Y' || chosen == 'B' { 2 } else { 3 }
}

fn score1(opponent: char, me: char) -> u32 {
    let score_win_draw = if win(opponent, me) { 6 } else if draw(opponent, me) { 3 } else { 0 };
    let score_chosen = score_chosen(me);
    score_win_draw + score_chosen
}

fn score_chosen2(opponent: char, what_to_do: char) -> u32 {
    match (opponent, what_to_do) {
        ('A', 'X') | ('B', 'Z') | ('C', 'Y') => score_chosen('C'),
        ('A', 'Y') | ('B', 'X') | ('C', 'Z') => score_chosen('A'),
        ('A', 'Z') | ('B', 'Y') | ('C', 'X') => score_chosen('B'),
        (_, _) => panic!("oh no!")
    }
}

fn score2(opponent: char, what_to_do: char) -> u32 {
    let win_draw = if what_to_do == 'Z' { 6 } else if what_to_do == 'Y' { 3 } else { 0 };
    let score_chosen = score_chosen2(opponent, what_to_do);
    win_draw + score_chosen
}