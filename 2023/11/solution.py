import itertools
from typing import Iterable

from utils import run


def transpose(matrix: Iterable[Iterable[str]]) -> Iterable[Iterable[int]]:
    return list(map(list, itertools.zip_longest(*matrix, fillvalue=None)))


def solve(input_data: str) -> Iterable[int]:
    initial_grid = [list(line) for line in input_data.splitlines()]
    empty_rows = set([i for i, row in enumerate(initial_grid) if all(v == '.' for v in row)])
    transposed_initial = transpose(initial_grid)
    empty_columns = set([j for j, col in enumerate(transposed_initial) if all(v == '.' for v in col)])

    galaxies = [(i, j) for i, row in enumerate(initial_grid) for j, v in enumerate(row) if v == '#']
    initial_path_sum = 0
    added_rows_cols = 0
    for (g1i, g1j), (g2i, g2j) in itertools.combinations(galaxies, 2):
        i_from = min(g1i, g2i)
        i_to = max(g1i, g2i)
        j_from = min(g1j, g2j)
        j_to = max(g1j, g2j)
        initial_path = (i_to - i_from) + (j_to - j_from)
        added_rows = len(set(range(i_from, i_to + 1)).intersection(empty_rows))
        added_columns = len(set(range(j_from, j_to + 1)).intersection(empty_columns))
        initial_path_sum += initial_path
        added_rows_cols += added_rows + added_columns
    part_1_sum = initial_path_sum + added_rows_cols
    part_2_sum = initial_path_sum + (1000000 - 1) * added_rows_cols

    yield part_1_sum
    yield part_2_sum


run(solve)
