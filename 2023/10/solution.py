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
    while new_neighbours := [neigh for neigh in neighbours(current) if neigh not in loop]:
        current = new_neighbours.pop()
        loop.add(current)

    yield len(loop) // 2

    # border = sorted(
    #     [spot for spots in [[(i, 0), (i, len(grid) - 1)] for i in range(len(grid[0]))]
    #      + [[(0, j), (len(grid[0]) - 1, j)] for j in range(len(grid))]
    #      for spot in spots]
    # )
    # outside_spot = [spot for spot in border if spot not in loop][0]
    # print(outside_spot)
    #
    # def all_neighbours(curr: tuple[int, int]) -> Iterable[tuple[int, int]]:
    #     ci, cj = curr
    #     for di, dj in [(-1, 0), (0, 1), (1, 0), (0, -1)]:
    #         if 0 <= ci + di < len(grid) and 0 <= cj + dj < len(grid[0]):
    #             yield ci + di, cj + dj
    #
    # outside_count = 1
    # visited = loop
    # current = outside_spot
    # while new_neighbours := [neigh for neigh in all_neighbours(current) if neigh not in visited]:
    #     current = new_neighbours.pop()
    #     outside_count += 1
    #
    # inside_count =
    # print(f"outside_count = {outside_count}, loop = {len(loop)} -> ")


run(solve)
