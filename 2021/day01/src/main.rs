// Code for the problem https://adventofcode.com/2021/day/1
const YEAR: u32 = 2021;
const DAY: u32 = 1;

fn main() {
    let input_lines: Vec<i32> = elf::get_input(YEAR, DAY, "<>");
    let (_, count1) = input_lines
        .iter()
        .fold((None, 0), |old, depth| match old.0 {
            Some(last) if last < *depth => (Some(*depth), old.1 + 1),
            _ => (Some(*depth), old.1)
        });
    println!("Part1: {}", count1);

    let (_, count2) = input_lines
        .as_slice()
        .windows(3)
        .fold((None, 0), |old, depths| match (old.0, depths.iter().sum::<i32>()) {
            (Some(last), new_sum) if last < new_sum => (Some(new_sum), old.1 + 1),
            (_, new_sum) => (Some(new_sum), old.1)
        });
    println!("Part2: {}", count2);
}
