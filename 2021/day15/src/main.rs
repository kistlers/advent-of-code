use fast_paths::InputGraph;

// Code for the problem https://adventofcode.com/2021/day/15
const YEAR: u32 = 2021;
const DAY: u32 = 15;

fn main() {
    let input_lines: Vec<String> = elf::get_input(YEAR, DAY, "53616c7465645f5fc3caba04b37cfdc549d664f17097aa5457af16a74fbc99744bebc15d2bc22c9784c1224456ac0746");

    let grid = input_lines.iter().fold(vec![], |mut grid, line| {
        grid.push(line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<_>>());
        grid
    });
    println!("Part1: {}", get_shortest_path(&grid));

    let rows = grid.len();
    let cols = grid[0].len();

    let mut large_grid = vec![vec![0; cols * 5]; rows * 5];
    for row in 0..rows {
        for col in 0..cols {
            large_grid[row][col] = grid[row][col];
        }
    }

    for col_block in 1..5 {
        for row in 0..rows {
            for col in 0..cols {
                large_grid[row][cols * col_block + col] =
                    match large_grid[row][cols * (col_block - 1) + col] {
                        9 => 1,
                        x => x + 1
                    };
            }
        }
    }

    for row_block in 1..5 {
        for col_block in 0..5 {
            for row in 0..rows {
                for col in 0..cols {
                    large_grid[rows * row_block + row][cols * col_block + col] =
                        match large_grid[rows * (row_block - 1) + row][cols * col_block + col] {
                            9 => 1,
                            x => x + 1
                        };
                }
            }
        }
    }
    println!("Part2: {}", get_shortest_path(&large_grid));
}

fn get_shortest_path(grid: &[Vec<usize>]) -> usize {
    let mut graph = InputGraph::new();
    let rows = grid.len();
    let cols = grid[0].len();
    for row in 0..rows {
        for col in 0..cols {
            if col > 0 {
                // from left
                graph.add_edge(row * cols + (col - 1), row * cols + col, grid[row][col]);
                // println!("({}, {}) to ({}, {}) w: {}", row, col - 1, row, col, grid[row][col]);
                // to left
                graph.add_edge(row * cols + col, row * cols + (col - 1), grid[row][col - 1]);
                // println!("({}, {}) to ({}, {}) w: {}", row, col, row, col - 1, grid[row][col - 1]);
            }
            if row > 0 {
                // from top
                graph.add_edge((row - 1) * cols + col, row * cols + col, grid[row][col]);
                // println!("({}, {}) to ({}, {}) w: {}", row - 1, col, row, col, grid[row][col]);
                // to top
                graph.add_edge(row * cols + col, (row - 1) * cols + col, grid[row - 1][col]);
                // println!("({}, {}) to ({}, {}) w: {}", row, col, row - 1, col, grid[row - 1][col]);
            }
        }
    }
    graph.freeze();
    let fast_graph = fast_paths::prepare(&graph);
    let shortest_path = fast_paths::calc_path(&fast_graph, 0, (rows - 1) * cols + (cols - 1));
    match shortest_path {
        Some(p) => p.get_weight(),
        None => panic!()
    }
}
