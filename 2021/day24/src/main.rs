use std::collections::{HashSet, VecDeque};
use itertools::Itertools;

// Code for the problem https://adventofcode.com/2021/day/24
const YEAR: u32 = 2021;
const DAY: u32 = 24;

const REGISTERS: usize = 4;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "53616c7465645f5fc3caba04b37cfdc549d664f17097aa5457af16a74fbc99744bebc15d2bc22c9784c1224456ac0746");

    println!("Part1: {}", find_extremum(true, &input_lines));
    println!("Part1: {}", find_extremum(false, &input_lines));
}


fn find_extremum(from_largest: bool, input_lines: &[String]) -> i64 {
    let mut queue = VecDeque::from([State::new()]);

    let mut visited = HashSet::new();

    while let Some(state) = queue.pop_front() {
        if visited.contains(&(state.line, state.get_reg("z"))) {
            continue;
        }

        if from_largest && state.number < 10 && state.number > 0 && state.number != 9 {
            // for my input the largest starts with a 9
            continue;
        }
        if !from_largest && state.number < 10 && state.number > 0 && state.number != 1 {
            // for my input the smallest starts with a 1
            continue;
        }

        if input_lines.len() == state.line {
            // break;
            if state.get_reg("z") == 0 {
                // valid
                return state.number;
            } else {
                // not valid
                continue;
            }
        }

        visited.insert((state.line, state.get_reg("z")));

        for new_state in process_next_digit(from_largest, input_lines, state) {
            queue.push_back(new_state);
        }
    }
    panic!();
}

fn process_next_digit(from_largest: bool, input_lines: &[String], mut state: State) -> Vec<State> {
    // println!("inp: {:?}", input_lines[state.line].split_whitespace().collect::<Vec<_>>());
    let (_, reg) = input_lines[state.line].split_whitespace().collect_tuple().unwrap();
    state.line += 1;
    let mut digits = (1i64..10i64).collect_vec();
    if from_largest {
        digits.reverse();
    }
    digits.iter().map(|digit| {
        let mut new_state = state;
        new_state.regs[reg_to_index(reg)] = *digit;
        run_alu_next_digit(input_lines, &mut new_state);
        new_state.number = 10 * new_state.number + digit;
        // println!("{:?}", new_state);
        new_state
    }).collect()
}

fn run_alu_next_digit(input_lines: &[String], mut state: &mut State) {
    while state.line < input_lines.len() && !input_lines[state.line].starts_with("inp") {
        // println!("instr: {:?}", input_lines[state.line].split_whitespace().collect::<Vec<_>>());
        let (instr, target, op) = input_lines[state.line].split_whitespace().collect_tuple().unwrap();
        let op_value = if op.chars().all(char::is_alphabetic) {
            state.get_reg(op)
        } else {
            op.parse::<i64>().unwrap()
        };

        let target_value = state.get_reg(target);
        let result = match instr {
            "add" => target_value + op_value,
            "mul" => target_value * op_value,
            "div" => round_towards_zero(target_value as f64 / op_value as f64),
            "mod" => target_value % op_value,
            "eql" => if target_value == op_value { 1 } else { 0 },
            o => panic!("unknown instr {}", o)
        };
        state.set_reg(target, result);

        state.line += 1;
    }
}

fn round_towards_zero(val: f64) -> i64 {
    if val < 0f64 {
        val.ceil() as i64
    } else {
        val.floor() as i64
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct State {
    regs: [i64; REGISTERS],
    number: i64,
    line: usize,
}

impl State {
    fn new() -> Self {
        Self { regs: [0, 0, 0, 0], number: 0, line: 0 }
    }

    fn get_reg(&self, reg_name: &str) -> i64 {
        self.regs[reg_to_index(reg_name)]
    }

    fn set_reg(&mut self, reg_name: &str, value: i64) {
        self.regs[reg_to_index(reg_name)] = value;
    }
}

fn reg_to_index(name: &str) -> usize {
    match name {
        "w" => 0,
        "x" => 1,
        "y" => 2,
        "z" => 3,
        o => panic!("unknown reg {}", o)
    }
}

