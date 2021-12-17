use std::cmp::max;

// Code for the problem https://adventofcode.com/2021/day/17
const YEAR: u32 = 2021;
const DAY: u32 = 17;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "53616c7465645f5fc3caba04b37cfdc549d664f17097aa5457af16a74fbc99744bebc15d2bc22c9784c1224456ac0746");
    let (x_target, y_target) = input_lines[0].trim_start_matches("target area: ").split_once(", ").unwrap();
    let (x_min, x_max) = x_target
        .trim_start_matches("x=")
        .split_once("..")
        .map(|(from, to)| (from.parse::<i32>().unwrap(), to.parse::<i32>().unwrap()))
        .unwrap();
    let (y_min, y_max) = y_target
        .trim_start_matches("y=")
        .split_once("..")
        .map(|(from, to)| (from.parse::<i32>().unwrap(), to.parse::<i32>().unwrap()))
        .unwrap();
    println!("({},{}) ({},{})", x_min, x_max, y_min, y_max);

    let (best_height, num_velocities) = get_best_height(x_min, x_max, y_min, y_max);
    println!("Part1: {}", best_height);
    println!("Part2: {}", num_velocities);
}

struct Loc {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    x: i32,
    y: i32,
    xv: i32,
    yv: i32,
    highest: i32,
}

impl Loc {
    pub(crate) fn will_not_reach(&self) -> bool {
        self.xv == 0 && self.x < self.x_min
    }
}

impl Loc {
    pub(crate) fn step_loc(&mut self) {
        self.x += self.xv;
        self.y += self.yv;
        if self.xv > 0 { self.xv -= 1 } else if self.xv < 0 { self.xv += 1 }
        self.yv -= 1;
        self.highest = max(self.highest, self.y)
    }
}

impl Loc {
    pub(crate) fn in_target(&self) -> bool {
        self.x_min <= self.x && self.x <= self.x_max && self.y_min <= self.y && self.y <= self.y_max
    }
}

impl Loc {
    pub(crate) fn did_overshoot(&self) -> bool {
        self.x > self.x_max || self.y < self.y_min
    }
}

fn get_best_height(x_min: i32, x_max: i32, y_min: i32, y_max: i32) -> (i32, i32) {
    let mut best_height = 0;
    let mut num_velocities = 0;
    for xv in 0..(x_max + 1) {
        // for xv in 7..8 {
        // println!("xv: {}", xv);
        for yv in y_min..5000 {
            // for yv in 2..3 {
            // println!("yv: {}", xv);
            let new_height = get_height(Loc { x_min, x_max, y_min, y_max, x: 0, y: 0, xv, yv, highest: 0 });
            // println!("best: {}, height for init v ({},{}): {}", best_height, xv, yv, new_height);
            best_height = max(
                best_height,
                new_height,
            );
            if new_height >= 0 {num_velocities += 1};
        }
    }
    (best_height, num_velocities)
}


fn get_height(mut loc: Loc) -> i32 {
    while !loc.did_overshoot() && !loc.will_not_reach() {
        // println!("v: ({},{}), at: ({},{}), target: ({}..{} , {}..{})", loc.xv, loc.yv, loc.x, loc.y, loc.x_min, loc.x_max, loc.y_min, loc.y_max);
        if loc.in_target() {
            return loc.highest;
        }
        loc.step_loc();
    }
    -1
}
