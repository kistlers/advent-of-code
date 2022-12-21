use std::collections::HashMap;
use elf::measure;
use itertools::Itertools;

// Code for the problem https://adventofcode.com/2022/day/17
const YEAR: u32 = 2022;
const DAY: u32 = 17;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "<>");
    let instructions = input_lines[0].chars().collect_vec();

    let (part1, part2) = measure!({
        let part1 = Chamber::new(instructions.clone()).solve_for_steps(2022);
        let part2 = Chamber::new(instructions).solve_for_steps(1_000_000_000_000);
        (part1, part2)
    });
    println!("part1: {part1}");
    println!("part2: {part2}");
}

#[derive(Clone, Debug)]
struct Chamber {
    grid: Vec<Vec<bool>>,
    instructions: Vec<char>,
    instr_ptr: usize,
    curr_height: usize,
    next_rock: Kind,
    cache: HashMap<(usize, usize), (usize, usize)>,
}

impl Chamber {
    fn solve_for_steps(&mut self, steps: usize) -> usize {
        for step in 0..steps {
            if let Some(final_height) = self.place_next_rock(step, steps) {
                return final_height;
            }
        }
        self.curr_height
    }

    fn place_next_rock(&mut self, curr_step: usize, steps: usize) -> Option<usize> {
        let kind = self.fetch_and_inc_next_kind();
        let mut x = 2;
        let mut y = self.curr_height + 3 + kind.get_added_y_top_left();

        if let std::collections::hash_map::Entry::Vacant(entry)
            = self.cache.entry((kind as usize, self.instr_ptr)) {
            entry.insert((curr_step, self.curr_height));
        } else {
            let (cached_step, cached_height) = self.cache[&(kind as usize, self.instr_ptr)];
            let repetition_length = curr_step - cached_step;
            let steps_to_go = steps - curr_step;
            if steps_to_go % repetition_length == 0 {
                return Some(self.curr_height + steps_to_go / repetition_length * (self.curr_height - cached_height));
            }
        }

        loop {
            let instruction = self.fetch_and_inc_instruction();
            x = self.push(&kind, x, y, instruction);
            // println!("{:?} at ({},{}) after {}", kind, x, y, instruction);
            // self.print_with_current(&kind, x, y);

            if self.collide_or_fall(&kind, x, y) {
                self.settle(&kind, x, y);
                break;
            }

            y -= 1;
        }
        self.curr_height = self.curr_height.max(y + 1);
        None
    }

    fn fetch_and_inc_next_kind(&mut self) -> Kind {
        let next_kind = self.next_rock.to_owned();
        self.next_rock = next_kind.next_kind();
        next_kind
    }

    #[allow(dead_code)]
    fn print_with_current(&self, kind: &Kind, x: usize, y: usize) {
        let mut pretty = self.grid.iter()
            .take(y + 1)
            .map(|row| row.iter().map(|p| if *p { '#' } else { '.' }).collect_vec())
            .collect_vec();
        for (dx, dy) in kind.get_locations() {
            pretty[y - dy][x + dx] = '@';
        }
        for row in pretty.iter().rev() {
            println!("|{}|", row.iter().join(""));
        }
        println!("+-------+");
        println!();
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row in self.grid.iter().take(self.curr_height + 2).rev() {
            println!("|{}|", row.iter().map(|p| if *p { '#' } else { '.' }).join(""));
        }
        println!("+-------+");
        println!();
    }

    // returns true if collided
    fn collide_or_fall(&self, kind: &Kind, x: usize, y: usize) -> bool {
        match kind {
            Kind::Dash =>
                y < 1
                    || self.grid[y - 1][x]
                    || self.grid[y - 1][x + 1]
                    || self.grid[y - 1][x + 2]
                    || self.grid[y - 1][x + 3],
            Kind::Plus =>
                y < 3
                    || self.grid[y - 2][x]
                    || self.grid[y - 3][x + 1]
                    || self.grid[y - 2][x + 2],
            Kind::Corner =>
                y < 3
                    || self.grid[y - 3][x]
                    || self.grid[y - 3][x + 1]
                    || self.grid[y - 3][x + 2],
            Kind::Bar =>
                y < 4
                    || self.grid[y - 4][x],
            Kind::Square =>
                y < 2
                    || self.grid[y - 2][x]
                    || self.grid[y - 2][x + 1]
        }
    }

    fn push(&self, kind: &Kind, x: usize, y: usize, instruction: char) -> usize {
        match (kind, instruction) {
            (Kind::Dash, '<') => {
                if x > 0 && !self.grid[y][x - 1] {
                    x - 1
                } else {
                    x
                }
            }
            (Kind::Dash, '>') => {
                if x < 3 && !self.grid[y][x + 4] {
                    x + 1
                } else {
                    x
                }
            }
            (Kind::Plus, '<') => {
                if x > 0
                    && !self.grid[y][x]
                    && !self.grid[y - 1][x - 1]
                    && !self.grid[y - 2][x] {
                    x - 1
                } else {
                    x
                }
            }
            (Kind::Plus, '>') => {
                if x < 4
                    && !self.grid[y][x + 2]
                    && !self.grid[y - 1][x + 3]
                    && !self.grid[y - 2][x + 2] {
                    x + 1
                } else {
                    x
                }
            }
            (Kind::Corner, '<') => {
                if x > 0
                    && !self.grid[y][x + 1]
                    && !self.grid[y - 1][x + 1]
                    && !self.grid[y - 2][x - 1] {
                    x - 1
                } else {
                    x
                }
            }
            (Kind::Corner, '>') => {
                if x < 4
                    && !self.grid[y][x + 3]
                    && !self.grid[y - 1][x + 3]
                    && !self.grid[y - 2][x + 3] {
                    x + 1
                } else {
                    x
                }
            }
            (Kind::Bar, '<') => {
                if x > 0
                    && !self.grid[y][x - 1]
                    && !self.grid[y - 1][x - 1]
                    && !self.grid[y - 2][x - 1]
                    && !self.grid[y - 3][x - 1] {
                    x - 1
                } else {
                    x
                }
            }
            (Kind::Bar, '>') => {
                if x < 6
                    && !self.grid[y][x + 1]
                    && !self.grid[y - 1][x + 1]
                    && !self.grid[y - 2][x + 1]
                    && !self.grid[y - 3][x + 1] {
                    x + 1
                } else {
                    x
                }
            }
            (Kind::Square, '<') => {
                if x > 0
                    && !self.grid[y][x - 1]
                    && !self.grid[y - 1][x - 1] {
                    x - 1
                } else {
                    x
                }
            }
            (Kind::Square, '>') => {
                if x < 5
                    && !self.grid[y][x + 2]
                    && !self.grid[y - 1][x + 2] {
                    x + 1
                } else {
                    x
                }
            }
            _ => panic!()
        }
    }
    fn settle(&mut self, kind: &Kind, x: usize, y: usize) {
        for (dx, dy) in kind.get_locations() {
            self.grid[y - dy][x + dx] = true;
        }
    }

    fn new(instructions: Vec<char>) -> Self {
        Self {
            grid: vec![vec![false; 7]; 5000],
            instructions,
            instr_ptr: 0,
            curr_height: 0,
            next_rock: Kind::Dash,
            cache: HashMap::new(),
        }
    }
    fn fetch_and_inc_instruction(&mut self) -> char {
        let instruction = self.instructions[self.instr_ptr];
        self.instr_ptr = (self.instr_ptr + 1) % self.instructions.len();
        instruction
    }
}

#[derive(Clone, Debug, Copy)]
enum Kind {
    Dash,
    Plus,
    Corner,
    Bar,
    Square,
}

impl Kind {
    fn get_locations(&self) -> Vec<(usize, usize)> { // (x, y)
        match self {
            Kind::Dash => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            Kind::Plus => vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            Kind::Corner => vec![(2, 0), (2, 1), (0, 2), (1, 2), (2, 2)],
            Kind::Bar => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            Kind::Square => vec![(0, 0), (1, 0), (0, 1), (1, 1)]
        }
    }

    fn get_added_y_top_left(&self) -> usize {
        match self {
            Kind::Dash => 0,
            Kind::Plus => 2,
            Kind::Corner => 2,
            Kind::Bar => 3,
            Kind::Square => 1
        }
    }

    fn next_kind(&self) -> Kind {
        match self {
            Kind::Dash => Kind::Plus,
            Kind::Plus => Kind::Corner,
            Kind::Corner => Kind::Bar,
            Kind::Bar => Kind::Square,
            Kind::Square => Kind::Dash
        }
    }
}
