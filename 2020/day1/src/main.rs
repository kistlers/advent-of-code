// Code for the problem https://adventofcode.com/2020/day/1
const YEAR: u32 = 2020;
const DAY: u32 = 1;

fn main() {
    let input_lines: Vec<i32> = elf::get_input(YEAR, DAY, "<>");
    for a in &input_lines {
        for b in &input_lines {
            if a + b == 2020 {
                println!("Part1: {}", a * b);
            } else if a + b < 2020 {
                for c in &input_lines {
                    if a + b + c == 2020 {
                        println!("Part2: {}", a * b * c);
                    }
                }
            }
        }
    }
}
