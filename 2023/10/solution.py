from typing import Iterable

from utils import run


def solve(input_data: str) -> Iterable[int]:
    grid = input_data.splitlines()

    si, sj = [(i, j) for i, row in enumerate(grid) for j, v in enumerate(row) if v == 'S'][0]

    symbol_to_dir = {
        'S': {(-1, 0), (0, 1), (1, 0), (0, -1)},
        '|': {(-1, 0), (1, 0)},
        '-': {(0, 1), (0, -1)},
        'L': {(-1, 0), (0, 1)},
        'J': {(-1, 0), (0, -1)},
        '7': {(1, 0), (0, -1)},
        'F': {(1, 0), (0, 1)},
        '.': set()
    }

    def neighbours(curr: tuple[int, int]) -> Iterable[tuple[int, int]]:
        ci, cj = curr
        for di, dj in symbol_to_dir.get(grid[ci][cj]):
            if 0 <= ci + di < len(grid) and 0 <= cj + dj < len(grid[0]):
                neighbour = grid[ci + di][cj + dj]
                if (-di, -dj) in symbol_to_dir.get(neighbour):
                    yield ci + di, cj + dj

    current = si, sj
    loop = {current}
    new_neighbours = set(neighbours(current))
    while new_neighbours:
        current = new_neighbours.pop()
        loop.add(current)
        new_neighbours = set(neighbours(current)).difference(loop)

    yield len(loop) // 2

    blown_up_grid = [['.'] * len(grid[0]) * 3 for _ in range(len(grid) * 3)]

    def all_neighbours_blown_up_grid(curr: tuple[int, int]) \
            -> Iterable[tuple[int, int]]:
        curri, currj = curr
        for di, dj in [(-1, 0), (0, 1), (1, 0), (0, -1)]:
            if 0 <= curri + di < len(blown_up_grid) and 0 <= currj + dj < len(blown_up_grid[0]):
                yield curri + di, currj + dj

    current = 0, 0
    outside_valid_count = 0

    to_visit = {current}
    while to_visit:
        ci, cj = current = to_visit.pop()
        if blown_up_grid[ci][cj] == 'X':
            continue
        if blown_up_grid[ci][cj] == '*':
            outside_valid_count += 1
        blown_up_grid[ci][cj] = 'X'
        to_visit = to_visit.union(all_neighbours_blown_up_grid(current))

    inside_count = len(grid) * len(grid[0]) - outside_valid_count - len(loop)
    yield inside_count


run(solve)
