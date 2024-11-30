use std::collections::{BTreeMap, BTreeSet};
use std::collections::btree_map::Entry::Vacant;
use std::str::FromStr;
use elf::measure;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

// Code for the problem https://adventofcode.com/2022/day/19
const YEAR: u32 = 2022;
const DAY: u32 = 19;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "<>");

    let blueprints: Vec<Blueprint> = input_lines.iter()
        .map(|input_line| input_line.parse::<Blueprint>().unwrap())
        .collect_vec();

    let (part1, part2) = measure!({
        (part1(&blueprints, 24), 0)
        // (part1(&blueprints, 24), part2(&blueprints, 32))
    });
    println!("part1: {part1}");
    println!("part2: {part2}");
}

fn part1(blueprints: &[Blueprint], time_limit: i32) -> u32 {
    blueprints.iter().enumerate()
        .map(|(id, b)| {
            let geodes = get_max_geodes(b, time_limit);
            let ql = (id as u32 + 1) * geodes;
            println!("id {}, geodes {} -> {}", id + 1, geodes, ql);
            ql
        })
        .sum::<u32>()
}

fn part2(blueprints: &[Blueprint], time_limit: i32) -> u32 {
    blueprints.iter().take(3)
        .map(|b| {
            let geodes = get_max_geodes(b, time_limit);
            println!("geodes {geodes}");
            geodes
        })
        .product::<u32>()
}

fn get_max_geodes(blueprint: &Blueprint, time_limit: i32) -> u32 {
    let mut states: BTreeMap<RobotState, BTreeSet<MineralState>> = BTreeMap::from([(RobotState::init(), BTreeSet::from([MineralState::new()]))]);

    for _ in 0..time_limit {
        let mut new_states: BTreeMap<RobotState, BTreeSet<MineralState>> = BTreeMap::new();

        for (robot_state, mineral_states) in &states {
            for mineral_state in mineral_states {
                if blueprint.can_build_geode_robot_at(mineral_state) {
                    let (new_robot_state, mut new_mineral_state) = blueprint.build_geode_robot(robot_state, mineral_state);
                    new_mineral_state.mine(robot_state);
                    insert_key_or_extend_value(&mut new_states, new_robot_state, new_mineral_state);
                }
                if blueprint.can_build_obsidian_robot_at(mineral_state) && mineral_state.obsidian < blueprint.max_obsidian_required {
                    let (new_robot_state, mut new_mineral_state) = blueprint.build_obsidian_robot(robot_state, mineral_state);
                    new_mineral_state.mine(robot_state);
                    insert_key_or_extend_value(&mut new_states, new_robot_state, new_mineral_state);
                }
                if blueprint.can_build_clay_robot_at(mineral_state) && mineral_state.clay < blueprint.max_clay_required {
                    let (new_robot_state, mut new_mineral_state) = blueprint.build_clay_robot(robot_state, mineral_state);
                    new_mineral_state.mine(robot_state);
                    insert_key_or_extend_value(&mut new_states, new_robot_state, new_mineral_state);
                }
                if blueprint.can_build_ore_robot_at(mineral_state) && mineral_state.ore < blueprint.max_ore_required {
                    let (new_robot_state, mut new_mineral_state) = blueprint.build_ore_robot(robot_state, mineral_state);
                    new_mineral_state.mine(robot_state);
                    insert_key_or_extend_value(&mut new_states, new_robot_state, new_mineral_state);
                }
                let mut new_mineral_state = *mineral_state;
                new_mineral_state.mine(robot_state);
                insert_key_or_extend_value(&mut new_states, *robot_state, new_mineral_state);
            }
        }

        println!("{:?}", new_states);
        states.clear();
        for (robots, new_mineral_states) in &new_states {
            states.insert(*robots, clean_mineral_states(new_mineral_states));
        }
        println!("{:?}", states);
        println!();
    }
    states.values().flatten().map(|mineral_state| mineral_state.geodes).max().unwrap()
}

fn insert_key_or_extend_value(new_states: &mut BTreeMap<RobotState, BTreeSet<MineralState>>, new_robot_state: RobotState, new_mineral_state: MineralState) {
    if let Vacant(entry) = new_states.entry(new_robot_state) {
        entry.insert(BTreeSet::from([new_mineral_state]));
    } else {
        new_states.get_mut(&new_robot_state).map(|val| val.insert(new_mineral_state));
    }
}

fn clean_mineral_states(states: &BTreeSet<MineralState>) -> BTreeSet<MineralState> {
    let mut keep_states = BTreeSet::new();
    for state in states {
        if states.iter()
            .filter(|s| *s != state)
            .filter(|s| s.all_better_than(&state))
            .count() == 0 {
            keep_states.insert(*state);
        }
    }
    keep_states
}

#[derive(Debug, Clone)]
struct Blueprint {
    ore_robot_ore: u32,
    clay_robot_ore: u32,
    obsidian_robot_ore: u32,
    obsidian_robot_clay: u32,
    geode_robot_ore: u32,
    geode_robot_obsidian: u32,
    max_ore_required: u32,
    max_clay_required: u32,
    max_obsidian_required: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct MineralState {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,
}

impl MineralState {
    fn mine(&mut self, robot_state: &RobotState) {
        self.ore += robot_state.ore_robots;
        self.clay += robot_state.clay_robots;
        self.obsidian += robot_state.obsidian_robots;
        self.geodes += robot_state.geode_robots;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct RobotState {
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
}

impl RobotState {
    fn init() -> Self {
        Self { ore_robots: 1, clay_robots: 0, obsidian_robots: 0, geode_robots: 0 }
    }
}

impl MineralState {
    fn all_better_than(&self, other: &&MineralState) -> bool {
        self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geodes >= other.geodes
    }

    fn new() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
        }
    }
}

impl Blueprint {
    fn build_geode_robot(&self, robot_state: &RobotState, mineral_state: &MineralState) -> (RobotState, MineralState) {
        let mut new_robot_state = *robot_state;
        let mut new_mineral_state = *mineral_state;
        new_mineral_state.ore -= self.geode_robot_ore;
        new_mineral_state.obsidian -= self.geode_robot_obsidian;
        new_robot_state.geode_robots += 1;
        (new_robot_state, new_mineral_state)
    }

    fn build_obsidian_robot(&self, robot_state: &RobotState, mineral_state: &MineralState) -> (RobotState, MineralState) {
        let mut new_robot_state = *robot_state;
        let mut new_mineral_state = *mineral_state;
        new_mineral_state.ore -= self.obsidian_robot_ore;
        new_mineral_state.clay -= self.obsidian_robot_clay;
        new_robot_state.obsidian_robots += 1;
        (new_robot_state, new_mineral_state)
    }

    fn build_clay_robot(&self, robot_state: &RobotState, mineral_state: &MineralState) -> (RobotState, MineralState) {
        let mut new_robot_state = *robot_state;
        let mut new_mineral_state = *mineral_state;
        new_mineral_state.ore -= self.clay_robot_ore;
        new_robot_state.clay_robots += 1;
        (new_robot_state, new_mineral_state)
    }

    fn build_ore_robot(&self, robot_state: &RobotState, mineral_state: &MineralState) -> (RobotState, MineralState) {
        let mut new_robot_state = *robot_state;
        let mut new_mineral_state = *mineral_state;
        new_mineral_state.ore -= self.ore_robot_ore;
        new_robot_state.ore_robots += 1;
        (new_robot_state, new_mineral_state)
    }

    fn can_build_ore_robot_at(&self, state: &MineralState) -> bool {
        state.ore >= self.ore_robot_ore
    }

    fn can_build_clay_robot_at(&self, state: &MineralState) -> bool {
        state.ore >= self.clay_robot_ore
    }

    fn can_build_obsidian_robot_at(&self, state: &MineralState) -> bool {
        state.ore >= self.obsidian_robot_ore && state.clay >= self.obsidian_robot_clay
    }

    fn can_build_geode_robot_at(&self, state: &MineralState) -> bool {
        state.ore >= self.geode_robot_ore && state.obsidian >= self.geode_robot_obsidian
    }

    fn new(
        ore_robot_ore: u32,
        clay_robot_ore: u32,
        obsidian_robot_ore: u32,
        obsidian_robot_clay: u32,
        geode_robot_ore: u32,
        geode_robot_obsidian: u32,
    ) -> Blueprint {
        Self {
            ore_robot_ore,
            clay_robot_ore,
            obsidian_robot_ore,
            obsidian_robot_clay,
            geode_robot_ore,
            geode_robot_obsidian,
            max_ore_required: ore_robot_ore.max(clay_robot_ore).max(obsidian_robot_ore).max(geode_robot_ore),
            max_clay_required: obsidian_robot_clay,
            max_obsidian_required: geode_robot_obsidian,
        }
    }
}

impl FromStr for Blueprint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
        static ref REGEX: Regex = Regex::new(r"Blueprint \d+: Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    }
        let blueprint = if let
            Some((ore_robot_ore, clay_robot_ore, obsidian_robot_ore, obsidian_robot_clay, geode_robot_ore, geode_robot_obsidian))
            = REGEX.captures(s).unwrap().iter().skip(1).map(|num| num.unwrap().as_str().parse::<u32>().unwrap()).collect_tuple()
        { Blueprint::new(ore_robot_ore, clay_robot_ore, obsidian_robot_ore, obsidian_robot_clay, geode_robot_ore, geode_robot_obsidian) } else { panic!() };
        Ok(blueprint)
    }
}
