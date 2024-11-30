use elf::SplitExtension;

// Code for the problem https://adventofcode.com/2021/day/2
const YEAR: u32 = 2021;
const DAY: u32 = 2;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "53616c7465645f5fc3caba04b37cfdc549d664f17097aa5457af16a74fbc99744bebc15d2bc22c9784c1224456ac0746");

    let (depth, forward) = &input_lines.iter().fold((0, 0), |(depth, forward), input_line| {
        let (dir, val): (_, i32) = input_line.split_once_parse_value(" ").unwrap();
        match dir {
            "down" => {
                (depth + val, forward)
            }
            "up" => {
                (depth - val, forward)
            }
            "forward" => {
                (depth, forward + val)
            }
            _ => panic!()
        }
    });
    println!("Part1: {}", forward * depth);


    let (depth, forward, _) = &input_lines.iter().fold((0, 0, 0), |(depth, forward, aim), input_line| {
        let (dir, val): (_, i32) = input_line.split_once_parse_value(" ").unwrap();
        match dir {
            "down" => {
                (depth, forward, aim + val)
            }
            "up" => {
                (depth, forward, aim - val)
            }
            "forward" => {
                (depth + aim * val, forward + val, aim)
            }
            _ => panic!()
        }
    });
    println!("Part2: {}", forward * depth);
}
