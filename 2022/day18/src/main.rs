use std::collections::VecDeque;
use elf::measure;
use itertools::Itertools;

// Code for the problem https://adventofcode.com/2022/day/18
const YEAR: u32 = 2022;
const DAY: u32 = 18;

const VECTORS: [(isize, isize, isize); 6] = [(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)];

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "<>");
    let cubes = input_lines.iter()
        .map(|input_line| {
            let (cx, cy, cz): (usize, usize, usize) = if let Some((x, y, z)) = input_line.split(',').map(|c| c.parse::<usize>().unwrap()).collect_tuple() { (x, y, z) } else { panic!() };
            (cx + 1, cy + 1, cz + 1) // boundary of size around teh droplet
        })
        .collect_vec();
    let (dimx, dimy, dimz) = cubes.iter()
        .copied()
        .reduce(|(lx, ly, lz), (rx, ry, rz)| (lx.max(rx), ly.max(ry), lz.max(rz)))
        .unwrap();
    let (dimx, dimy, dimz) = (dimx + 2, dimy + 2, dimz + 2);
    let mut grid = vec![vec![vec![false; dimz]; dimy]; dimx];
    for (cx, cy, cz) in &cubes {
        grid[*cx][*cy][*cz] = true;
    }

    let (part1, part2) = measure!({
        let mut visited = vec![vec![vec![false; dimz]; dimy]; dimx];
        let part1 = cubes.iter().map(|c| bfs(&grid, &mut visited, dimx, dimy, dimz, *c, false)).sum::<usize>();

        let mut visited = vec![vec![vec![false; dimz]; dimy]; dimx];
        let part2 = bfs(&grid, &mut visited, dimx, dimy, dimz, (0, 0, 0), true);

        (part1, part2)
    });
    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

fn bfs(
    grid: &[Vec<Vec<bool>>],
    visited: &mut [Vec<Vec<bool>>],
    dimx: usize,
    dimy: usize,
    dimz: usize,
    start: (usize, usize, usize),
    count_cubes: bool,
) -> usize {
    let mut queue = VecDeque::from([start]);
    let mut open_faces = 0;

    while !queue.is_empty() {
        let (cx, cy, cz) = queue.pop_front().unwrap();
        if visited[cx][cy][cz] {
            continue;
        }
        visited[cx][cy][cz] = true;
        for (x, y, z) in legal_neighbours(cx, cy, cz, dimx, dimy, dimz) {
            if grid[x][y][z] == count_cubes {
                // a cube
                open_faces += 1;
            } else if grid[x][y][z] != count_cubes {
                // not a cube
                queue.push_back((x, y, z));
            }
        }
    }
    open_faces
}

fn legal_neighbours(
    cx: usize,
    cy: usize,
    cz: usize,
    dimx: usize,
    dimy: usize,
    dimz: usize,
) -> Vec<(usize, usize, usize)> {
    VECTORS.iter().map(|(dx, dy, dz)|
        (cx as isize + dx, cy as isize + dy, cz as isize + dz))
        .filter(|(x, y, z)| *x >= 0 && *y >= 0 && *z >= 0)
        .map(|(x, y, z)| (x as usize, y as usize, z as usize))
        .filter(|(x, y, z)| *x < dimx && *y < dimy && *z < dimz)
        .collect_vec()
}
