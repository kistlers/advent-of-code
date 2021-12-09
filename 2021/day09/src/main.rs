// Code for the problem https://adventofcode.com/2021/day/9
const YEAR: u32 = 2021;
const DAY: u32 = 9;

const D_VECTORS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "53616c7465645f5fc3caba04b37cfdc549d664f17097aa5457af16a74fbc99744bebc15d2bc22c9784c1224456ac0746");
    let width = input_lines[0].len();
    let height = input_lines.len();
    let mut points = vec![vec![u32::MAX; width + 2]; height + 2];
    for h in 1..=height {
        points[h].splice(
            1..=width,
            input_lines[h - 1]
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>(),
        );
    }

    let mut risk1 = 0;
    for h in 1..=height {
        for w in 1..=width {
            let curr = points[h][w];
            if curr < points[h][w - 1] && curr < points[h][w + 1] && curr < points[h - 1][w] && curr < points[h + 1][w] {
                risk1 += points[h][w] + 1;
            }
        }
    }
    println!("Part1: {}", risk1);

    let mut basins = vec![];
    for h in 1..=height {
        for w in 1..=width {
            let curr = points[h][w];
            if curr < points[h][w - 1] && curr < points[h][w + 1] && curr < points[h - 1][w] && curr < points[h + 1][w] {
                basins.push(get_size_basin(&mut points, h, w, height, width));
            }
        }
    }
    basins.sort_unstable();
    basins.reverse();
    println!("Part2: {}", basins[0] * basins[1] * basins[2]);
}

fn get_size_basin(points: &mut Vec<Vec<u32>>, h_start: usize, w_start: usize, height: usize, width: usize) -> u32 {
    let mut size = 0;
    let mut stack = vec![(h_start, w_start); 1];
    let mut visited = vec![vec![false; width + 2]; height + 2];
    while !stack.is_empty() {
        let (h, w) = stack.pop().unwrap();
        if points[h][w] == 9 || visited[h][w] || h == 0 || w == 0 || h == height + 1 || w == width + 1 {
            continue;
        }
        visited[h][w] = true;
        size += 1;
        for (dh, dw) in D_VECTORS {
            stack.push(((h as i32 + dh) as usize, (w as i32 + dw) as usize));
        }
    }
    size
}
