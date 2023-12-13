import copy
import itertools
from functools import reduce
from typing import Iterable

from utils import run


def transpose(matrix: list[list[str]]) -> list[list[str]]:
    return list(map(list, itertools.zip_longest(*matrix, fillvalue=None)))


def find_lines_above_mirrored(lines: list[list[str]], ignore_reflection_line: int = -1) -> int:
    n_lines = len(lines) - 1
    for i in range(n_lines):
        if lines[i] == lines[i + 1]:
            aligned = list(zip(reversed(lines[:i + 1]), lines[i + 1:]))
            matches = [left == right for left, right in aligned]
            try:
                matching_lines = matches.index(False)
            except ValueError:
                matching_lines = len(matches)

            if matching_lines == i + 1 or n_lines - i == matching_lines:
                reflection_line = i + 1
                if not reflection_line == ignore_reflection_line:
                    return i + 1

    return 0


def switched_symbols(matrix: list[list[str]]) -> list[tuple[int, int, str]]:
    return [(i, j, '.' if cell == '#' else '#') for i, row in enumerate(matrix) for j, cell in enumerate(row)]


def solve(input_data: str) -> Iterable[int]:
    patterns = [[list(row) for row in pattern.splitlines()] for pattern in input_data.split("\n\n")]

    part_1 = 0
    part_2 = 0
    for pattern_i, pattern in enumerate(patterns):
        rows = pattern
        columns = transpose(rows)
        new_lines_above_mirrored_rows = find_lines_above_mirrored(rows)
        new_left_of_above_mirrored_columns = find_lines_above_mirrored(columns)
        if new_lines_above_mirrored_rows and not new_left_of_above_mirrored_columns:
            part_1 += 100 * new_lines_above_mirrored_rows
        elif not new_lines_above_mirrored_rows and new_left_of_above_mirrored_columns:
            part_1 += new_left_of_above_mirrored_columns
        else:
            return Exception("should not be here")

        for i, j, new_symbol in switched_symbols(pattern):
            rows_copy = copy.deepcopy(rows)
            rows_copy[i][j] = new_symbol
            columns_copy = copy.deepcopy(columns)
            columns_copy[j][i] = new_symbol

            new_lines_above_mirrored_rows_copy = (
                find_lines_above_mirrored(rows_copy, new_lines_above_mirrored_rows))
            new_left_of_above_mirrored_columns_copy = (
                find_lines_above_mirrored(columns_copy, new_left_of_above_mirrored_columns))
            before = (new_lines_above_mirrored_rows, new_left_of_above_mirrored_columns)
            after = (new_lines_above_mirrored_rows_copy, new_left_of_above_mirrored_columns_copy)
            if after and after != before:
                if new_lines_above_mirrored_rows_copy != 0:
                    part_2 += 100 * new_lines_above_mirrored_rows_copy
                    break
                elif new_left_of_above_mirrored_columns_copy != 0:
                    part_2 += new_left_of_above_mirrored_columns_copy
                    break
        else:
            raise Exception("should not be here")

    yield part_1
    yield part_2


run(solve)
