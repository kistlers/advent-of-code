use std::cmp;
use elf::regex::Regex;
use itertools::Itertools;

// Code for the problem https://adventofcode.com/2021/day/21
const YEAR: u32 = 2021;
const DAY: u32 = 21;

const SPACES: usize = 10;
const PART1_LIMIT: usize = 1000;

const PART2_LIMIT: usize = 21;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "53616c7465645f5fc3caba04b37cfdc549d664f17097aa5457af16a74fbc99744bebc15d2bc22c9784c1224456ac0746");
    let regex = Regex::new(r"Player \d starting position: (?P<pos>\d+)").unwrap();

    let (player0, player1) = input_lines.iter().map(|line| {
        let captures = regex.captures(line).unwrap();
        captures.name("pos").unwrap().as_str().parse::<usize>().unwrap() - 1
    }).collect_tuple().unwrap();

    println!("Part1: {}", part1(player0, player1));
    println!("Part2: {}", part2(player0, player1));
}

fn part1(mut player0: usize, mut player1: usize) -> usize {
    let mut dice = 1usize;
    let mut score0 = 0;
    let mut score1 = 0;

    loop {
        let rolls = 3 * dice + 3;
        dice += 3;
        player0 += rolls;
        player0 %= SPACES;
        score0 += player0 + 1;
        if score0 >= PART1_LIMIT {
            break;
        }

        let rolls = 3 * dice + 3;
        dice += 3;
        player1 += rolls;
        player1 %= SPACES;
        score1 += player1 + 1;
        if score1 >= PART1_LIMIT {
            break;
        }
    }

    cmp::min(score0, score1) * (dice - 1)
}

fn part2(player0: usize, player1: usize) -> usize {
    let mut memo: MemoTable = vec![vec![vec![vec![vec![(0usize, 0usize); 2]; PART2_LIMIT]; PART2_LIMIT]; SPACES]; SPACES];
    let dirac_wins = dirac(&mut memo, player0, player1, 0, 0, 0);
    cmp::max(dirac_wins.0, dirac_wins.1)
}

fn dirac(memo: &mut MemoTable, player0: usize, player1: usize, score0: usize, score1: usize, turn_of_player: usize) -> (usize, usize) {
    if score0 >= PART2_LIMIT {
        return (1, 0);
    }
    if score1 >= PART2_LIMIT {
        return (0, 1);
    }
    let memo_val = memo[player0][player1][score0][score1][turn_of_player];
    if memo_val != (0, 0) {
        return memo_val;
    }

    let mut dirac_wins = (0, 0);
    if turn_of_player == 0 {
        for i in 1usize..=3usize {
            for j in 1usize..=3usize {
                for k in 1usize..=3usize {
                    let pos = (player0 + i + j + k) % SPACES;
                    let wins = dirac(memo, pos, player1, score0 + pos + 1, score1, (turn_of_player + 1) % 2);
                    dirac_wins.0 += wins.0;
                    dirac_wins.1 += wins.1;
                }
            }
        }
    } else {
        for i in 1usize..=3usize {
            for j in 1usize..=3usize {
                for k in 1usize..=3usize {
                    let pos = (player1 + i + j + k) % SPACES;
                    let wins = dirac(memo, player0, pos, score0, score1 + pos + 1, (turn_of_player + 1) % 2);
                    dirac_wins.0 += wins.0;
                    dirac_wins.1 += wins.1;
                }
            }
        }
    }

    memo[player0][player1][score0][score1][turn_of_player] = dirac_wins;
    dirac_wins
}

type MemoTable = Vec<Vec<Vec<Vec<Vec<(usize, usize)>>>>>;
