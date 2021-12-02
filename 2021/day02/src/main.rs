// Code for the problem https://adventofcode.com/2021/day/2
const YEAR: u32 = 2021;
const DAY: u32 = 2;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "53616c7465645f5fc3caba04b37cfdc549d664f17097aa5457af16a74fbc99744bebc15d2bc22c9784c1224456ac0746");
    let mut depth1 = 0;
    let mut forward1 = 0;

    let mut aim2 = 0;
    let mut depth2 = 0;
    let mut forward2 = 0;

    for input_line in &input_lines {
        let pattern: Vec<_> = input_line.split(" ").collect();
        let dir = pattern[0];
        let val = pattern[1].parse::<i32>().unwrap();
        match dir {
            "down" => {
                depth1 += val;
                aim2 += val;
            }
            "up" => {
                depth1 -= val;
                aim2 -= val;
            }
            "forward" => {
                forward1 += val;
                forward2 += val;
                depth2 += aim2 * val;
            }
            _ => panic!()
        }
    }
    println!("Part1: {}", forward1 * depth1);
    println!("Part2: {}", forward2 * depth2);
}
