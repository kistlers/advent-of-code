use itertools::Itertools;

// Code for the problem https://adventofcode.com/2022/day/05
const YEAR: u32 = 2022;
const DAY: u32 = 05;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "<>");
    let move_lines = input_lines.iter()
        .skip_while(|input_line| !input_line.starts_with("move"))
        .cloned()
        .map(|move_line| {
            if let Some((num, from, to)) = move_line
                .split(' ')
                .skip(1)
                .step_by(2)
                .map(|i| i.parse::<usize>().unwrap())
                .collect_tuple() {
                (num, from, to)
            } else {
                panic!()
            }
        })
        .collect::<Vec<(usize, usize, usize)>>();

    /*    let mut stacks1 = vec![
            "ZN".chars().collect::<Vec<char>>(),
            "MCD".chars().collect::<Vec<char>>(),
            "P".chars().collect::<Vec<char>>(),
        ];*/
    let mut stacks1 = vec![
        "BSVZGPW".chars().collect::<Vec<char>>(),
        "JVBCZF".chars().collect::<Vec<char>>(),
        "VLMHNZDC".chars().collect::<Vec<char>>(),
        "LDMZPFJB".chars().collect::<Vec<char>>(),
        "VFCGJBQH".chars().collect::<Vec<char>>(),
        "GFQTSLB".chars().collect::<Vec<char>>(),
        "LGCZV".chars().collect::<Vec<char>>(),
        "NLG".chars().collect::<Vec<char>>(),
        "JFHC".chars().collect::<Vec<char>>(),
    ];
    let mut stacks2 = stacks1.clone();

    for (num, from, to) in move_lines {
        let mut items2: Vec<char> = vec![];

        for _ in 0..num {
            let item = stacks1[from - 1].pop().unwrap();
            stacks1[to - 1].push(item);
            items2.push(item);
        }

        items2.reverse();
        stacks2[to - 1].append(&mut items2);
    }

    let part1 = stacks1.iter().map(|stack| stack.last().unwrap()).join("");
    println!("part1: {}", part1);

    let part2 = stacks2.iter().map(|stack| stack.last().unwrap()).join("");
    println!("part2: {}", part2);
}
