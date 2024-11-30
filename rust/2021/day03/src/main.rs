use std::convert::TryInto;

// Code for the problem https://adventofcode.com/2021/day/3
const YEAR: u32 = 2021;
const DAY: u32 = 3;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "53616c7465645f5fc3caba04b37cfdc549d664f17097aa5457af16a74fbc99744bebc15d2bc22c9784c1224456ac0746");
    let element_len = input_lines[0].len();
    let mut accumulators = vec![(0, 0); input_lines[0].len()];
    for value in &input_lines {
        let bits: Vec<char> = value.chars().collect();
        for i in 0..element_len {
            match bits[i] {
                '0' => {
                    let a = accumulators.get(i).unwrap();
                    accumulators[i] = (a.0 + 1, a.1);
                }
                '1' => {
                    let a = accumulators.get(i).unwrap();
                    accumulators[i] = (a.0, a.1 + 1);
                }
                _ => panic!()
            }
        }
    }

    let init: (String, String) = (String::from(""), String::from(""));
    let (gamma, epsilon) = accumulators.iter().fold(init, |(gamma, epsilon), acc| {
        if acc.0 > acc.1 {
            (gamma + "0", epsilon + "1")
        } else {
            (gamma + "1", epsilon + "0")
        }
    });
    let gamma = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon, 2).unwrap();
    println!("Part1: {}, gamma {}, epsilon {}", gamma * epsilon, gamma, epsilon);

    let mut oxygen = input_lines.clone();
    let mut co2 = input_lines.clone();
    for i in 0..element_len {
        let oxygen_len: u32 = oxygen.len().try_into().unwrap();
        if oxygen_len > 1 {
            let ones = oxygen.iter().map(|ox| ox.chars().nth(i).map(|c| c.to_digit(2).unwrap()).unwrap()).sum::<u32>();
            let keep = if ones >= oxygen_len - ones { '1' } else { '0' };
            oxygen.retain(|ox| *(ox.chars().collect::<Vec<char>>().get(i).unwrap()) == keep);
        }

        let co2_len: u32 = co2.len().try_into().unwrap();
        if co2_len > 1 {
            let ones = co2.iter().map(|co| co.chars().nth(i).map(|c| c.to_digit(2).unwrap()).unwrap()).sum::<u32>();
            let keep = if ones < co2_len - ones { '1' } else { '0' };
            co2.retain(|co| *(co.chars().collect::<Vec<char>>().get(i).unwrap()) == keep);
        }
    }
    let oxygen = isize::from_str_radix(oxygen.get(0).unwrap(), 2).unwrap();
    let co2 = isize::from_str_radix(co2.get(0).unwrap(), 2).unwrap();
    println!("Part2: {}, oxygen {}, co2 {}", oxygen * co2, oxygen, co2);
}
