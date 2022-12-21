use std::collections::HashMap;
use std::fmt::Error;
use std::str::FromStr;
use elf::measure;
use itertools::Itertools;

// Code for the problem https://adventofcode.com/2022/day/21
const YEAR: u32 = 2022;
const DAY: u32 = 21;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "<>");
    let monkeys = input_lines.iter()
        .map(|input_line| {
            let monkey = input_line.parse::<Monkey>().unwrap();
            (monkey.name.clone(), monkey)
        }).collect::<HashMap<_, _>>();
    let (part1, part2) = measure!({
        let part1 = evaluate(&monkeys["root"], &monkeys);
        let part2 = match_values_calc_human(0, &monkeys["root"], &monkeys);
        (part1, part2)
    });
    println!("part1: {part1}");
    println!("part2: {part2}");
}

fn evaluate(curr: &Monkey, monkeys: &HashMap<String, Monkey>) -> i64 {
    match curr.expr_type {
        ExprType::Val => curr.compute_const(),
        _ => {
            let left_value = evaluate(&monkeys[&curr.left_operand.clone().unwrap()], monkeys);
            let right_value = evaluate(&monkeys[&curr.right_operand.clone().unwrap()], monkeys);
            curr.compute_expr(left_value, right_value)
        }
    }
}

fn match_values_calc_human(expected_value: i64, curr: &Monkey, monkeys: &HashMap<String, Monkey>) -> i64 {
    if curr.name == "humn" {
        return expected_value;
    }

    if curr.expr_type == ExprType::Val {
        return curr.compute_const();
    }

    let left_monkey = &monkeys[&curr.left_operand.clone().unwrap()];
    let right_monkey = &monkeys[&curr.right_operand.clone().unwrap()];
    let left_value = evaluate_or_none(left_monkey, monkeys);
    let right_value = evaluate_or_none(right_monkey, monkeys);
    if let Some(left_value) = left_value {
        // right is None
        let new_expected_value = if curr.name == "root" {
            left_value
        } else {
            match curr.expr_type {
                ExprType::Add => expected_value - left_value, // expected = left + x
                ExprType::Sub => left_value - expected_value, // expected = left - x
                ExprType::Mul => expected_value / left_value, // expected = left * x
                ExprType::Div => left_value / expected_value, // expected = left / x
                _ => panic!(),
            }
        };
        match_values_calc_human(new_expected_value, right_monkey, monkeys)
    } else if let Some(right_value) = right_value {
        // left is None
        let new_expected_value = if curr.name == "root" {
            right_value
        } else {
            match curr.expr_type {
                ExprType::Add => expected_value - right_value, // expected = x + right
                ExprType::Sub => expected_value + right_value, // expected = x - right
                ExprType::Mul => expected_value / right_value, // expected = x * right
                ExprType::Div => expected_value * right_value, // expected = x / right
                _ => panic!(),
            }
        };
        match_values_calc_human(new_expected_value, left_monkey, monkeys)
    } else {
        panic!()
    }
}

fn evaluate_or_none(curr: &Monkey, monkeys: &HashMap<String, Monkey>) -> Option<i64> {
    if curr.name == "humn" {
        None
    } else {
        match curr.expr_type {
            ExprType::Val => Some(curr.compute_const()),
            _ => {
                let left_value = evaluate_or_none(&monkeys[&curr.left_operand.clone().unwrap()], monkeys);
                let right_value = evaluate_or_none(&monkeys[&curr.right_operand.clone().unwrap()], monkeys);
                match (left_value, right_value) {
                    (None, None) => panic!(),
                    (Some(_), None) => None,
                    (None, Some(_)) => None,
                    (Some(left_value), Some(right_value)) => Some(curr.compute_expr(left_value, right_value))
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ExprType {
    Val,
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
struct Monkey {
    name: String,
    value: i64,
    expr_type: ExprType,
    left_operand: Option<String>,
    right_operand: Option<String>,
}

impl Monkey {
    fn compute_const(&self) -> i64 {
        self.value
    }

    fn compute_expr(&self, left_value: i64, right_value: i64) -> i64 {
        match self.expr_type {
            ExprType::Add => left_value + right_value,
            ExprType::Sub => left_value - right_value,
            ExprType::Mul => left_value * right_value,
            ExprType::Div => left_value / right_value,
            _ => panic!(),
        }
    }

    fn new(
        name: String,
        value: i64,
        expr_type: ExprType,
        left_operand: Option<String>,
        right_operand: Option<String>,
    ) -> Self {
        Self { name, value, expr_type, left_operand, right_operand }
    }
}

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, expr_string) = s.split_once(": ").unwrap();
        let (value, left_operand, expr_type, right_operand) = if expr_string.contains(' ') {
            (0, Some(expr_string.split(' ').collect_vec()[0].to_string()), expr_string.split(' ').collect_vec()[1].to_string().parse::<ExprType>().unwrap(), Some(expr_string.split(' ').collect_vec()[2].to_string()))
        } else { (expr_string.parse().unwrap(), None, ExprType::Val, None) };
        Ok(Monkey::new(name.to_string(), value, expr_type, left_operand, right_operand))
    }
}

impl FromStr for ExprType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(ExprType::Add),
            "-" => Ok(ExprType::Sub),
            "*" => Ok(ExprType::Mul),
            "/" => Ok(ExprType::Div),
            _ => panic!(),
        }
    }
}