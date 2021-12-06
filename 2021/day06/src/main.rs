// Code for the problem https://adventofcode.com/2021/day/6
const YEAR: u32 = 2021;
const DAY: u32 = 6;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "53616c7465645f5fc3caba04b37cfdc549d664f17097aa5457af16a74fbc99744bebc15d2bc22c9784c1224456ac0746");
    let init_state = input_lines.get(0).unwrap().split(",").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let mut state = init_state.clone();

    for _i in 0..80 {
        // println!("i: {}", i);
        // for s in &state {
        //     print!("{},", s);
        // }
        let mut new_state = vec![];

        for s in state.iter_mut() {
            // print!("*s: {},", *s);
            if *s == 0 {
                *s = 6;
                new_state.push(8);
            } else {
                *s -= 1;
            }
            // print!("*s: {},", *s);
        }
        // println!();
        // for s in &new_state {
        //     print!("{},", s);
        // }
        state.extend(new_state);
        // println!();
    }
    println!("Part1: {}", state.len());
}
