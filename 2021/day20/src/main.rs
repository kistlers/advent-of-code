use elf::measure;

// Code for the problem https://adventofcode.com/2021/day/20
const YEAR: u32 = 2021;
const DAY: u32 = 20;

const D_VECTORS: [(isize, isize); 9] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 0), (0, 1), (1, -1), (1, 0), (1, 1)];

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "53616c7465645f5fc3caba04b37cfdc549d664f17097aa5457af16a74fbc99744bebc15d2bc22c9784c1224456ac0746");
    let mut part1_count = 0;
    let mut part2_count = 0;

    measure!({
        let lookup = input_lines[0].chars().map(char_to_int).collect::<Vec<_>>();
        let mut image = input_lines[2..]
            .iter()
            .map(|line| line.chars().map(char_to_int).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        for step in 1..=50 {
            let outside_pixel = if step % 2 == 0 && lookup[0] == 1 { 1 } else { 0 };
            image = update_image(&image, &lookup, outside_pixel);
            if step == 2 {
                part1_count = count_pixels(&image);
            }
            if step == 50 {
                part2_count = count_pixels(&image);
            }
        }
    });
    println!("Part1: {}", part1_count);
    println!("Part2: {}", part2_count);
}

fn update_image(image: &[Vec<i32>], lookup: &[i32], outside_pixel: i32) -> Vec<Vec<i32>> {
    let mut new_image = vec![vec![0; image[0].len() + 2]; image.len() + 2];
    let rows = new_image.len() as isize;
    let cols = new_image[0].len() as isize;
    for row in -1..rows - 1 {
        for col in -1..cols - 1 {
            let mut lookup_index = 0;
            for (d_row, d_col) in D_VECTORS {
                let row_coord = row + d_row;
                let col_coord = col + d_col;
                lookup_index <<= 1;
                lookup_index |= if row_coord < 0 || col_coord < 0 || row_coord >= rows - 2 || col_coord >= cols - 2 {
                    outside_pixel
                } else {
                    image[row_coord as usize][col_coord as usize]
                };
            }
            new_image[(row + 1) as usize][(col + 1) as usize] = lookup[lookup_index as usize];
        }
    }
    new_image
}

fn count_pixels(image: &[Vec<i32>]) -> i32 {
    image.iter().flatten().fold(0, |lit, pixel| if *pixel == 0 { lit } else { lit + 1 })
}

fn char_to_int(c: char) -> i32 {
    match c {
        '.' => 0,
        '#' => 1,
        _ => panic!()
    }
}
