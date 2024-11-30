use elf::measure;

// Code for the problem https://adventofcode.com/2021/day/10
const YEAR: u32 = 2021;
const DAY: u32 = 10;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "53616c7465645f5fc3caba04b37cfdc549d664f17097aa5457af16a74fbc99744bebc15d2bc22c9784c1224456ac0746");
    measure!({
        let mut error_sum = 0;
        let mut completion_scores: Vec<usize> = vec![];
        for input_line in input_lines {
            let mut stack: Vec<char> = vec![];
            let mut is_broken = false;
            let mut local_score: usize = 0;
            for char in input_line.chars() {
                match char {
                    '(' => stack.push(char),
                    '[' => stack.push(char),
                    '{' => stack.push(char),
                    '<' => stack.push(char),
                    ')' => if stack.pop() == Some('(') {} else {
                        error_sum += get_error_value(')');
                        is_broken = true;
                        break;
                    },
                    ']' => if stack.pop() == Some('[') {} else {
                        error_sum += get_error_value(']');
                        is_broken = true;
                        break;
                    },
                    '}' => if stack.pop() == Some('{') {} else {
                        error_sum += get_error_value('}');
                        is_broken = true;
                        break;
                    },
                    '>' => if stack.pop() == Some('<') {} else {
                        error_sum += get_error_value('>');
                        is_broken = true;
                        break;
                    },
                    _ => panic!()
                }
            }
            if !is_broken {
                while !stack.is_empty() {
                    let top = stack.pop().unwrap();
                    local_score = local_score * 5 + get_completion_value(top);
                }
                completion_scores.push(local_score);
            }
        }

        println!("Part1: {}", error_sum);

        completion_scores.sort_unstable();
        println!("Part2: {}", completion_scores.get((completion_scores.len() - 1) / 2).unwrap());
    });
}

fn get_completion_value(char: char) -> usize {
    match char {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!()
    }
}

fn get_error_value(char: char) -> i32 {
    match char {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!()
    }
}
