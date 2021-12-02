use elf::regex::Regex;

// Code for the problem https://adventofcode.com/2020/day/2
const YEAR: u32 = 2020;
const DAY: u32 = 2;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "<>");
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    let mut valid1 = 0;
    for input_line in &input_lines {
        for cap in re.captures_iter(input_line) {
            let min = cap[1].parse::<i32>().unwrap();
            let max = cap[2].parse::<i32>().unwrap();
            let character = cap[3].parse::<char>().unwrap();
            let password = &cap[4];
            let actual = password.chars().fold(0, |count, c| if c == character { count + 1 } else { count });
            if min <= actual && actual <= max {
                valid1 += 1;
            }
        }
    }
    println!("Part1: {}", valid1);

    let mut valid2 = 0;
    for input_line in &input_lines {
        for cap in re.captures_iter(input_line) {
            let min = cap[1].parse::<usize>().unwrap();
            let max = cap[2].parse::<usize>().unwrap();
            let character = cap[3].parse::<char>().unwrap();
            let password = &cap[4];
            if (password.chars().nth(min - 1) == Some(character)) != (password.chars().nth(max - 1) == Some(character)) {
                valid2 += 1;
            }
        }
    }
    println!("Part2: {}", valid2);
}
