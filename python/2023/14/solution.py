from typing import Iterable

from python.utils import run

n_rows = 0
n_cols = 0


def tilt_north(tuple_grid: tuple[tuple[str, ...], ...]) -> tuple[tuple[str, ...], ...]:
    grid = list(list(row) for row in tuple_grid)
    for j in range(n_cols):
        for i1 in range(n_rows - 1):
            if grid[i1][j] != '.':
                continue
            for i2 in range(i1 + 1, n_rows):
                if grid[i2][j] == '#':
                    break
                if grid[i1][j] == '.' and grid[i2][j] == 'O':
                    grid[i1][j], grid[i2][j] = grid[i2][j], grid[i1][j]
    return tuple(tuple(row) for row in grid)


def tilt_south(tuple_grid: tuple[tuple[str, ...], ...]) -> tuple[tuple[str, ...], ...]:
    grid = list(list(row) for row in tuple_grid)
    for j in range(n_cols):
        for i1 in reversed(range(1, n_rows)):
            if grid[i1][j] != '.':
                continue
            for i2 in reversed(range(i1)):
                if grid[i2][j] == '#':
                    break
                if grid[i1][j] == '.' and grid[i2][j] == 'O':
                    grid[i1][j], grid[i2][j] = grid[i2][j], grid[i1][j]
    return tuple(tuple(row) for row in grid)


def tilt_west(tuple_grid: tuple[tuple[str, ...], ...]) -> tuple[tuple[str, ...], ...]:
    grid = list(list(row) for row in tuple_grid)
    for i in range(n_rows):
        for j1 in range(n_cols - 1):
            if grid[i][j1] != '.':
                continue
            for j2 in range(j1 + 1, n_cols):
                if grid[i][j2] == '#':
                    break
                if grid[i][j1] == '.' and grid[i][j2] == 'O':
                    grid[i][j1], grid[i][j2] = grid[i][j2], grid[i][j1]
    return tuple(tuple(row) for row in grid)


def tilt_east(tuple_grid: tuple[tuple[str, ...], ...]) -> tuple[tuple[str, ...], ...]:
    grid = list(list(row) for row in tuple_grid)
    for i in range(n_rows):
        for j1 in reversed(range(1, n_cols)):
            if grid[i][j1] != '.':
                continue
            for j2 in reversed(range(j1)):
                if grid[i][j2] == '#':
                    break
                if grid[i][j1] == '.' and grid[i][j2] == 'O':
                    grid[i][j1], grid[i][j2] = grid[i][j2], grid[i][j1]
    return tuple(tuple(row) for row in grid)


def calculate_load(grid: tuple[tuple[str, ...], ...]) -> int:
    return sum([i + 1 for i, row in enumerate(reversed(grid)) for cell in row if cell == 'O'])


def tilt_cycle(grid: tuple[tuple[str, ...], ...]) -> tuple[tuple[str, ...], ...]:
    return tilt_east(tilt_south(tilt_west(tilt_north(grid))))


def solve(input_data: str) -> Iterable[int]:
    grid: tuple[tuple[str, ...], ...] = tuple(tuple(line) for line in input_data.splitlines())
    global n_rows, n_cols
    n_rows = len(grid)
    n_cols = len(grid[0])

    yield calculate_load(tilt_north(grid))

    current_cycle = 0
    total_cycles = 1000000000
    cycle_len = None
    memo: dict[tuple[tuple[str, ...], ...], int] = dict()
    while current_cycle < total_cycles:
        if grid in memo and cycle_len is None:
            cycle_len = current_cycle - memo[grid]
            print(f"Cycle len: {cycle_len} at cycle {current_cycle}")
            current_cycle += cycle_len * ((total_cycles - current_cycle) // cycle_len)
        else:
            memo[grid] = current_cycle
            grid = tilt_cycle(grid)
            current_cycle += 1

    yield calculate_load(grid)


run(solve)
