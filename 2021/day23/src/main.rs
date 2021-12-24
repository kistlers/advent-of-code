use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};
use std::str::FromStr;
use std::hash::Hash;

// Code for the problem https://adventofcode.com/2021/day/23
const YEAR: u32 = 2021;
const DAY: u32 = 23;

const AMPHIPOD_A: Amphipod = Amphipod { kind: AmphipodKind::A, done: false };
const AMPHIPOD_B: Amphipod = Amphipod { kind: AmphipodKind::B, done: false };
const AMPHIPOD_C: Amphipod = Amphipod { kind: AmphipodKind::C, done: false };
const AMPHIPOD_D: Amphipod = Amphipod { kind: AmphipodKind::D, done: false };

const HALLWAY_LEN: usize = 11;
const ROOMS: usize = 4;
const ROOM_ENTRIES: [usize; 4] = [2, 4, 6, 8];

const EMPTY_HALLWAY: [Option<Amphipod>; HALLWAY_LEN] = [None, None, None, None, None, None, None, None, None, None, None];
const TARGET_ROW: [Option<Amphipod>; ROOMS] = [Some(AMPHIPOD_A), Some(AMPHIPOD_B), Some(AMPHIPOD_C), Some(AMPHIPOD_D)];

const PART2_UPPER_ROW: [Option<Amphipod>; ROOMS] = [Some(AMPHIPOD_D), Some(AMPHIPOD_C), Some(AMPHIPOD_B), Some(AMPHIPOD_A)];
const PART2_LOWER_ROW: [Option<Amphipod>; ROOMS] = [Some(AMPHIPOD_D), Some(AMPHIPOD_B), Some(AMPHIPOD_A), Some(AMPHIPOD_C)];

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "53616c7465645f5fc3caba04b37cfdc549d664f17097aa5457af16a74fbc99744bebc15d2bc22c9784c1224456ac0746");

    let mut rows: Vec<[Option<Amphipod>; ROOMS]> = [Default::default(), Default::default()].to_vec();
    input_lines
        .iter()
        .skip(2)
        .enumerate()
        .for_each(|(row, line)| line
            .split('#')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<Amphipod>().unwrap())
            .enumerate()
            .for_each(|(i, amphipod)| rows[row][i] = Some(amphipod))
        );

    let mut start1 = State::new(rows.clone());
    start1.initialize_done_state();
    let mut target1 = State::new(Vec::from([TARGET_ROW, TARGET_ROW]));
    target1.initialize_done_state();

    let mut min_score_per_state: HashMap<State, i64> = HashMap::from([(start1.clone(), 0)]);
    find_cheapest(&mut start1, &mut min_score_per_state);
    println!("Part1: {}", min_score_per_state[&target1]);

    let mut start2 = State::new(Vec::from([rows[0], PART2_UPPER_ROW, PART2_LOWER_ROW, rows[1]]));
    start2.initialize_done_state();
    let mut target2 = State::new(Vec::from([TARGET_ROW, TARGET_ROW, TARGET_ROW, TARGET_ROW]));
    target2.initialize_done_state();

    let mut min_score_per_state: HashMap<State, i64> = HashMap::from([(start2.clone(), 0)]);
    find_cheapest(&mut start2, &mut min_score_per_state);
    println!("Part2: {}", min_score_per_state[&target2]);
}

fn find_cheapest(state: &mut State, min_score_per_state: &mut HashMap<State, i64>) {
    if state.all_done() {
        return;
    }

    // hallway to room
    for (hallway_loc, target_room, room_depth, amphipod) in state.get_all_can_move_hallway_to_room() {
        let mut new_state = state.clone();
        let step_cost = new_state.move_hallway_to_room_depth_get_cost(amphipod, hallway_loc, target_room, room_depth);
        let new_cost = min_score_per_state[state] + step_cost;
        update_and_recurse(&mut new_state, min_score_per_state, new_cost);
    }

    // room to hallway
    for (room, room_depth, target_hallway_loc, amphipod) in state.get_all_can_move_room_to_hallway() {
        let mut new_state = state.clone();
        let step_cost = new_state.move_room_depth_to_hallway_get_cost(amphipod, room, room_depth, target_hallway_loc);
        let new_cost = min_score_per_state[state] + step_cost;
        update_and_recurse(&mut new_state, min_score_per_state, new_cost);
    }
}

fn update_and_recurse(state: &mut State, min_score_per_state: &mut HashMap<State, i64>, new_cost: i64) {
    if min_score_per_state.get(state).is_none() || new_cost < min_score_per_state[state] {
        min_score_per_state.insert(state.clone(), new_cost);
        find_cheapest(state, min_score_per_state);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    hallway: [Option<Amphipod>; HALLWAY_LEN],
    rows: Vec<[Option<Amphipod>; ROOMS]>,
    num_rows: usize,
}

impl State {
    fn new(rows: Vec<[Option<Amphipod>; ROOMS]>) -> Self {
        Self { hallway: EMPTY_HALLWAY, num_rows: rows.len(), rows }
    }

    // check (from, to] or [to, from)
    fn can_move_in_hallway(&self, from: usize, to: usize) -> bool {
        match from.cmp(&to) {
            Ordering::Less => self.hallway[from + 1..=to].iter().all(|loc| loc.is_none()),
            Ordering::Greater => self.hallway[to..from].iter().all(|loc| loc.is_none()),
            Ordering::Equal => true
        }
    }

    // returns (hallway_loc, target_room, room_depth, amphipod)
    fn get_all_can_move_hallway_to_room(&self) -> Vec<(usize, usize, usize, Amphipod)> {
        self.hallway
            .iter()
            .enumerate()
            .filter(|(_, amphipod)| amphipod.is_some())
            .map(|(hallway_loc, amphipod)| (hallway_loc, amphipod.unwrap()))
            .filter(|(hallway_loc, amphipod)| self.can_move_in_hallway(*hallway_loc, ROOM_ENTRIES[amphipod.target_room()]))
            .map(|(hallway_loc, amphipod)| (hallway_loc, amphipod.target_room(), amphipod))
            .filter(|(_, target_room, amphipod)| self.room_accepts(*target_room, amphipod))
            .map(|(hallway_loc, target_room, amphipod)| {
                let room_depth = self.get_room(target_room)
                    .iter()
                    .enumerate()
                    .rev()
                    .filter(|(_, x)| x.is_none())
                    .map(|(room_depth, _)| room_depth)
                    .next()
                    .unwrap();
                (hallway_loc, target_room, room_depth, amphipod)
            })
            .collect()
    }

    // returns (room, room_depth, target_hallway_loc, amphipod)
    fn get_all_can_move_room_to_hallway(&self) -> Vec<(usize, usize, usize, Amphipod)> {
        (0..ROOMS)
            .map(|room| (room, self.rows
                .iter()
                .enumerate()
                .map(|(room_depth, row)| (room_depth, row[room]))
                .filter(|(_, amphipod)| amphipod.is_some() && !amphipod.unwrap().done)
                .map(|(room_depth, amphipod)| (room_depth, amphipod.unwrap()))
                .next()))
            .filter(|(_, room_depth_amphipod_option)| room_depth_amphipod_option.is_some())
            .map(|(room, room_depth_amphipod_option)| (room, room_depth_amphipod_option.unwrap()))
            .filter(|(_, (_, amphipod))| !amphipod.done)
            .map(|(room, (room_depth, amphipod))| self.available_hallway_locs()
                .iter()
                .filter(|hallway_loc| self.can_move_in_hallway(ROOM_ENTRIES[room], **hallway_loc))
                .map(|hallway_loc| (room, room_depth, *hallway_loc, amphipod))
                .collect::<Vec<_>>()
            )
            .flatten()
            .collect()
    }

    fn move_hallway_to_room_depth_get_cost(&mut self, mut amphipod: Amphipod, hallway_loc: usize, room: usize, room_depth: usize) -> i64 {
        amphipod.done = true;
        self.rows[room_depth][room] = Some(amphipod);
        self.hallway[hallway_loc] = None;

        self.get_move_cost(&amphipod, room, room_depth, hallway_loc)
    }

    fn move_room_depth_to_hallway_get_cost(&mut self, amphipod: Amphipod, room: usize, room_depth: usize, hallway_loc: usize) -> i64 {
        self.hallway[hallway_loc] = Some(amphipod);
        self.rows[room_depth][room] = None;

        self.get_move_cost(&amphipod, room, room_depth, hallway_loc)
    }

    fn get_move_cost(&self, amphipod: &Amphipod, room: usize, room_depth: usize, hallway_loc: usize) -> i64 {
        let unit_cost = (ROOM_ENTRIES[room] as i64 - hallway_loc as i64).abs() + room_depth as i64 + 1;
        unit_cost * amphipod.cost()
    }

    fn initialize_done_state(&mut self) {
        for room in 0..ROOMS {
            for room_depth in (0..self.num_rows).rev() {
                if self.rows[room_depth][room].is_some()
                    && self.rows[room_depth][room].unwrap().target_room() == room {
                    self.rows[room_depth][room] = self.rows[room_depth][room].map(|mut amphipod| {
                        amphipod.done = true;
                        amphipod
                    });
                } else {
                    break; // all above are also not done
                }
            }
        }
    }

    fn all_done(&self) -> bool {
        self.hallway.iter().all(|x| x.is_none())
            && self.rows.iter().flatten().all(|x| if let Some(x) = x { x.done } else { false })
    }

    fn available_hallway_locs(&self) -> Vec<usize> {
        (0..HALLWAY_LEN)
            .filter(|loc| !ROOM_ENTRIES.contains(loc))
            .filter(|loc| self.hallway[*loc].is_none())
            .collect()
    }

    fn room_accepts(&self, room: usize, amphipod: &Amphipod) -> bool {
        !self.get_room(room).iter().any(|other| other.is_some() && other.unwrap().kind != amphipod.kind)
    }

    fn get_room(&self, room: usize) -> Vec<Option<Amphipod>> {
        self.rows.iter().map(|row| row[room]).collect()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum AmphipodKind {
    A,
    B,
    C,
    D,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Amphipod {
    kind: AmphipodKind,
    done: bool,
}

impl Amphipod {
    fn new(kind: AmphipodKind) -> Self {
        Self { kind, done: false }
    }

    fn to_str_lowercase_done(self) -> &'static str {
        match self.kind {
            AmphipodKind::A if self.done => "a",
            AmphipodKind::A => "A",
            AmphipodKind::B if self.done => "b",
            AmphipodKind::B => "B",
            AmphipodKind::C if self.done => "c",
            AmphipodKind::C => "C",
            AmphipodKind::D if self.done => "d",
            AmphipodKind::D => "D"
        }
    }

    fn cost(&self) -> i64 {
        match self.kind {
            AmphipodKind::A => 1,
            AmphipodKind::B => 10,
            AmphipodKind::C => 100,
            AmphipodKind::D => 1000
        }
    }

    fn target_room(&self) -> usize {
        match self.kind {
            AmphipodKind::A => 0,
            AmphipodKind::B => 1,
            AmphipodKind::C => 2,
            AmphipodKind::D => 3
        }
    }
}

impl FromStr for Amphipod {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Amphipod::new(AmphipodKind::A)),
            "B" => Ok(Amphipod::new(AmphipodKind::B)),
            "C" => Ok(Amphipod::new(AmphipodKind::C)),
            "D" => Ok(Amphipod::new(AmphipodKind::D)),
            o => panic!("cannot parse '{}' as Amphipod", o)
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Err(error) = writeln!(f, "#{}#", self.hallway.map(|amphipod| amphipod_to_str(&amphipod)).join("")) {
            panic!("{}", error);
        }
        for row in self.rows.iter() {
            if let Err(error) = writeln!(f, "  #{}#  ", row.map(|amphipod| amphipod_to_str(&amphipod)).join("#")) {
                panic!("{}", error);
            }
        }
        Ok(())
    }
}

fn amphipod_to_str(option_amphipod: &Option<Amphipod>) -> &'static str {
    match option_amphipod {
        Some(amphipod) => amphipod.to_str_lowercase_done(),
        None => "."
    }
}