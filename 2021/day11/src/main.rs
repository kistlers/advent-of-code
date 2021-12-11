use elf::measure;

// Code for the problem https://adventofcode.com/2021/day/11
const YEAR: u32 = 2021;
const DAY: u32 = 11;

const D_VECTORS: [(i32, i32); 8] = [(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)];

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "53616c7465645f5fc3caba04b37cfdc549d664f17097aa5457af16a74fbc99744bebc15d2bc22c9784c1224456ac0746");

    let mut flashes = 0;
    let mut first_all_flash = -1;
    measure!({
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
        for step in 0.. {
            let mut init_flashes = vec![];
            for h in 1..=height {
                for w in 1..=width {
                    points[h][w] += 1;
                    if points[h][w] > 9 {
                        init_flashes.push((h, w));
                    }
                }
            }
            bfs_step(&mut points, &mut init_flashes, height, width);
            let mut all_flash = true;
            for h in 1..=height {
                for w in 1..=width {
                    if points[h][w] > 9 {
                        points[h][w] = 0;
                        if step < 100 {
                            flashes += 1;
                        }
                    } else {
                        all_flash = false;
                    }
                }
            }
            if first_all_flash == -1 && all_flash {
                first_all_flash = step + 1;
            }
            if first_all_flash >= 0 && step > 100 {
                break;
            }
        }
    });
    println!("Part1: {}", flashes);
    println!("Part2: {}", first_all_flash);
}

fn bfs_step(points: &mut Vec<Vec<u32>>, to_flash: &mut Vec<(usize, usize)>, height: usize, width: usize) {
    let mut flashed = vec![vec![false; width + 2]; height + 2];
    while !to_flash.is_empty() {
        let (h, w) = to_flash.pop().unwrap();
        if flashed[h][w] || h == 0 || w == 0 || h == height + 1 || w == width + 1 {
            continue;
        }
        if points[h][w] > 9 {
            flashed[h][w] = true;
            for (dh, dw) in D_VECTORS {
                let new_h = (h as i32 + dh) as usize;
                let new_w = (w as i32 + dw) as usize;
                points[new_h][new_w] += 1;
                to_flash.push((new_h as usize, new_w));
            }
        }
    }
}
