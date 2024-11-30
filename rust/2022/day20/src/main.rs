use elf::measure;
use itertools::Itertools;

// Code for the problem https://adventofcode.com/2022/day/20
const YEAR: u32 = 2022;
const DAY: u32 = 20;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "<>");
    let numbers = input_lines.iter().enumerate()
        .map(|(index, input_line)| (index + 1, input_line.parse::<i64>().unwrap()))
        .collect_vec();

    let (part1, part2) = measure!({
        let part2_numbers = numbers.iter().map(|(index, n)| (*index, n * 811589153)).collect_vec();
        (mix_and_get_coordinates(numbers, 1), mix_and_get_coordinates(part2_numbers, 10))
    });
    println!("part1: {part1}");
    println!("part2: {part2}");
}

fn coordinates(numbers: &Vec<(usize, i64)>) -> i64 {
    let index0 = numbers.iter().position(|&n| n.1 == 0).unwrap();
    let numbers_len = numbers.len();
    numbers[(index0 + 1000).rem_euclid(numbers_len)].1 + numbers[(index0 + 2000).rem_euclid(numbers_len)].1 + numbers[(index0 + 3000).rem_euclid(numbers_len)].1
}

fn mix_and_get_coordinates(mut numbers: Vec<(usize, i64)>, mix_times: usize) -> i64 {
    for _ in 0..mix_times {
        for i in 1..=numbers.len() {
            let index = numbers.iter().position(|&n| n.0 == i).unwrap();
            move_number_by(&mut numbers, index);
        }
    }
    coordinates(&numbers)
}

fn move_number_by(numbers: &mut Vec<(usize, i64)>, index: usize) {
    let numbers_len = numbers.len();
    // positive modulo to only get a single wrap-around and always move right
    let move_by = numbers[index].1.rem_euclid(numbers_len as i64 - 1);
    for (l, r) in tuple_windows_up(index, index + move_by as usize, numbers_len) {
        // println!("swap {:?} at index {l} with {:?} at index {r}", numbers[l.rem_euclid(numbers_len)], numbers[r.rem_euclid(numbers_len)]);
        numbers.swap(l.rem_euclid(numbers_len), r.rem_euclid(numbers_len));
    }
}

fn tuple_windows_up(from: usize, to: usize, modulo: usize) -> Vec<(usize, usize)> {
    (from..=to).map(|i| i.rem_euclid(modulo)).tuple_windows::<(usize, usize)>().collect_vec()
}