use std::collections::VecDeque;
use std::fmt::Error;
use std::str::FromStr;
use itertools::Itertools;
use eval::{Expr};
use num::integer::lcm;

// Code for the problem https://adventofcode.com/2022/day/11
const YEAR: u32 = 2022;
const DAY: u32 = 11;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "<>");

    let monkeys = Monkeys::new(
        input_lines
            .join("\n")
            .split("\n\n")
            .map(|monkey_lines| monkey_lines.parse::<Monkey>().unwrap())
            .collect_vec());

    let part1 = calculate(monkeys.clone(), 20, 3);
    println!("part1: {}", part1);
    let part2 = calculate(monkeys, 10000, 1);
    println!("part2: {}", part2);
}

fn calculate(mut monkeys: Monkeys, rounds: usize, divide_by: usize) -> usize {
    monkeys.monkeys.iter()
        .map(|monkey| monkey.divisible_by)
        .reduce(lcm)
        .unwrap();

    for _ in 0..rounds {
        monkeys.round(divide_by);
    }

    monkeys.monkeys.iter()
        .map(|monkey| monkey.inspections)
        .sorted_by(|a, b| b.cmp(a))
        .take(2)
        .product::<usize>()
}

#[derive(Debug, Clone)]
struct Monkeys {
    monkeys: Vec<Monkey>,
    lcm: usize,
}

impl Monkeys {
    fn round(&mut self, divide_by: usize) {
        for mi in 0..self.len() {
            let monkey = &mut self.monkeys[mi];
            monkey.inspections += monkey.items.len();

            let mut moves: Vec<(usize, usize)> = vec![];
            for _ in 0..monkey.items.len() {
                let item = monkey.pop_front_compute_worry() / divide_by % self.lcm;
                let if_true = monkey.if_true;
                let if_false = monkey.if_false;
                if item % monkey.divisible_by == 0 {
                    moves.push((if_true, item));
                } else {
                    moves.push((if_false, item));
                }
            }
            for (to, worry) in moves {
                self.push_back_to(to, worry);
            }
        }
    }

    fn new(monkeys: Vec<Monkey>) -> Self {
        let lcm = monkeys.iter()
            .map(|monkey| monkey.divisible_by)
            .reduce(lcm)
            .unwrap();
        Monkeys { monkeys, lcm }
    }

    fn len(&self) -> usize {
        self.monkeys.len()
    }

    fn push_back_to(&mut self, to: usize, item: usize) {
        self.monkeys[to].push_back(item)
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Expr,
    divisible_by: usize,
    if_true: usize,
    if_false: usize,
    inspections: usize,
}

impl Monkey {
    pub(crate) fn push_back(&mut self, item: usize) {
        self.items.push_back(item);
    }
}

impl Monkey {
    fn pop_front_compute_worry(&mut self) -> usize {
        let item = self.items.pop_front().unwrap();
        self.operation.clone()
            .value("old", item)
            .exec().unwrap()
            .as_u64().unwrap() as usize
    }

    fn new(
        items: VecDeque<usize>,
        operation: Expr,
        divisible_by: usize,
        if_true: usize,
        if_false: usize,
    ) -> Monkey {
        Monkey { items, operation, divisible_by, if_true, if_false, inspections: 0 }
    }
}

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.split('\n').collect_vec();
        let items = lines[1].split(": ").last().unwrap()
            .split(", ")
            .map(|item| item.parse::<usize>().unwrap())
            .collect::<VecDeque<_>>();
        let operation = Expr::new(lines[2].split(" = ").last().unwrap());
        let divisible_by = lines[3].split(" by ").last().unwrap().parse::<usize>().unwrap();
        let if_true = lines[4].split(" monkey ").last().unwrap().parse::<usize>().unwrap();
        let if_false = lines[5].split(" monkey ").last().unwrap().parse::<usize>().unwrap();
        Ok(Monkey::new(items, operation, divisible_by, if_true, if_false))
    }
}
