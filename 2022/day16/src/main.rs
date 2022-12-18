use std::collections::{BTreeMap, BTreeSet};
use elf::measure;
use lazy_static::lazy_static;
use regex::Regex;
use itertools::Itertools;

// Code for the problem https://adventofcode.com/2022/day/16
const YEAR: u32 = 2022;
const DAY: u32 = 16;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "<>");

    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? ([\w, ]+)").unwrap();
    }
    let valves = input_lines.iter()
        .map(|input_line| {
            let captures = REGEX.captures(input_line).unwrap().iter().skip(1).collect_vec();
            let name: &str = captures[0].unwrap().as_str();
            let rate: i64 = captures[1].unwrap().as_str().parse::<i64>().unwrap();
            let neighbours: Vec<&str> = captures[2].unwrap().as_str().split(", ").collect_vec();
            (name, (rate, neighbours))
        })
        .collect::<BTreeMap<_, _>>();

    let (part1, part2) = measure!({
        let valves_with_rate = valves.values().filter(|(rate, _)| *rate >0).count();
        let mut network = Network::new(&valves, valves_with_rate);
        let part1 = network.release( "AA", BTreeSet::new(), 30, false);
        println!("part1: {}", part1);

        // let mut network = Network::new(&valves, valves_with_rate);
        let part2 = network.release("AA", BTreeSet::new(), 26, true);
        (part1, part2)
    });
    println!("part2: {}", part2); // should be 2741
}


#[derive(Debug, Clone)]
struct Network<'a> {
    memo: BTreeMap<(&'a str, BTreeSet<&'a str>, usize, bool), i64>,
    valves: &'a BTreeMap<&'a str, (i64, Vec<&'a str>)>,
    valves_with_rate: usize,
}

impl<'a> Network<'a> {
    fn new(valves: &'a BTreeMap<&str, (i64, Vec<&str>)>, valves_with_rate: usize) -> Self {
        Self { valves, memo: BTreeMap::new(), valves_with_rate }
    }

    fn release(
        &mut self,
        valve: &'a str,
        open_valves: BTreeSet<&'a str>,
        minutes_left: usize,
        with_elephant: bool,
    ) -> i64 {
        if minutes_left == 0 {
            return if with_elephant {
                self.release("AA", open_valves.clone(), 26, false)
            } else {
                0
            };
        }

        if self.memo.contains_key(&(valve, open_valves.clone(), minutes_left, with_elephant)) {
            return self.memo[&(valve, open_valves, minutes_left, with_elephant)];
        }

        if self.valves_with_rate == open_valves.len() {
            return 0;
        }

        let mut best = 0;
        if self.valves[&valve].0 > 0 && !open_valves.contains(&valve) {
            let mut new_open_valves = open_valves.clone();
            new_open_valves.insert(valve);
            let added_score = (minutes_left as i64 - 1) * self.valves[&valve].0;
            best = added_score + self.release(valve, new_open_valves, minutes_left - 1, with_elephant);
        }

        for next_valve in self.valves[&valve].1.iter() {
            best = best.max(self.release(next_valve, open_valves.clone(), minutes_left - 1, with_elephant))
        }

        self.memo.insert((valve, open_valves.clone(), minutes_left, with_elephant), best);

        // self.global_best = self.global_best.max(best);
        // println!("{}", self.global_best);
        best
    }
}