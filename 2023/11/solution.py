import itertools
from typing import Iterable

from utils import run


def transpose(list_of_list: list[list[str]]):
    return list(map(list, itertools.zip_longest(*list_of_list, fillvalue=None)))


def solve(input_data: str) -> Iterable[int]:
    initial_grid = [list(line) for line in input_data.splitlines()]
    empty_rows = set([i for i, row in enumerate(initial_grid) if all(v == '.' for v in row)])
    print(empty_rows)
    transposed_initial = transpose(initial_grid)
    empty_columns = set([j for j, col in enumerate(transposed_initial) if all(v == '.' for v in col)])
    print(empty_columns)

    for row in initial_grid:
        print("".join(row))

    print()

    grid = initial_grid
    for i in sorted(empty_rows, reverse=True):
        grid.insert(i, ['.'] * len(grid[0]))
    grid = transpose(grid)
    for j in sorted(empty_columns, reverse=True):
        grid.insert(j, ['.'] * len(grid[0]))
    grid = transpose(grid)

    for row in grid:
        print("".join(row))

    galaxies = [(i, j) for i, row in enumerate(grid) for j, v in enumerate(row) if v == '#']
    path_sum = 0
    for (g1i, g1j), (g2i, g2j) in itertools.combinations(galaxies, 2):
        path = abs(g2i - g1i) + abs(g2j - g1j)
        path_sum += path
        print(
            f"({g1i}, {g1j}) -> ({g2i}, {g2j}) =  {path}")
    print(path_sum)
    yield path_sum


run(solve)
