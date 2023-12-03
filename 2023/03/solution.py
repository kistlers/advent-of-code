from __future__ import annotations  # allow circular type hints

import math
from dataclasses import dataclass, field
from typing import Iterable
from utils import run
import re


@dataclass
class Symbol:
    symbol: str
    loc: (int, int)
    # default values are stored as class, not instance variables when not using default_factory
    adjacent_to: list[Part] = field(default_factory=list)


@dataclass
class Part:
    part_number: int
    adjacent_to: list[(int, int)]

    def is_adjacent_to(self, symbol: Symbol):
        return symbol.loc in self.adjacent_to


def create_part(match: re.Match, row) -> Part:
    start_col, end_col = match.span()
    adjacent_to = [(row, start_col - 1), (row, end_col)]

    for col in range(start_col - 1, end_col + 1):
        adjacent_to.append((row - 1, col))
        adjacent_to.append((row + 1, col))

    part_number = int(match.group())
    return Part(part_number, adjacent_to)


def solve(input_data: str) -> Iterable[int]:
    lines = input_data.splitlines()

    parts = []
    symbols = []

    for row, line in enumerate(lines):
        part_number_matches_iter = re.finditer(r'(\d+)', line)
        parts += [create_part(match, row) for match in part_number_matches_iter]

        symbol_matches_iter = re.finditer(r'([^0-9.])', line)
        symbols += [Symbol(match.group(1), (row, match.start())) for match in symbol_matches_iter]

    part_1 = 0
    part_2 = 0

    for part in parts:
        adjacent_symbols = [symbol for symbol in symbols if part.is_adjacent_to(symbol)]
        if any(adjacent_symbols):
            part_1 += part.part_number

        for symbol in adjacent_symbols:
            if symbol.symbol == '*':
                symbol.adjacent_to.append(part)

    for symbol in symbols:
        if symbol.symbol == '*' and len(symbol.adjacent_to) == 2:
            gear_ratio = math.prod(part.part_number for part in symbol.adjacent_to)
            part_2 += gear_ratio

    yield part_1
    yield part_2


run(solve)
