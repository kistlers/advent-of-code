from copy import deepcopy
from typing import Iterable
import numpy as np

from python.utils import run

Days = dict[tuple[int, int], set[tuple[int, int]]]


def solve(input_data: str) -> Iterable[int]:
    grid = input_data.splitlines()
    side_len = len(grid)
    if side_len != len(grid[0]):
        raise ValueError(f"len(grid): {len(grid)} != {len(grid[0])} :len(grid[0])")

    if side_len == 11:
        # only 6 days for sample part 1 and 5000 for part 2
        days_part_1 = 6
    else:
        days_part_1 = 64

    start = [(i, j) for i, row in enumerate(grid) for j, cell in enumerate(row) if cell == 'S'][0]
    initial: Days = dict(
        ((i, j), set()) for i, row in enumerate(grid) for j, cell in enumerate(row))

    current_day = deepcopy(initial)
    current_day[start] = {(0, 0)}

    def neighbours(curr: tuple[int, int]) -> Iterable[tuple[tuple[int, int], tuple[int, int]]]:
        ci, cj = curr
        for di, dj in [(1, 0), (-1, 0), (0, 1), (0, -1)]:
            cdi = ci + di
            cdj = cj + dj
            cdi_mod = cdi % side_len
            cdj_mod = cdj % side_len
            neighbour = grid[cdi_mod][cdj_mod]
            if neighbour == '#':
                continue
            elif cdi < 0:
                yield (cdi_mod, cdj_mod), (-1, 0)
            elif cdi >= side_len:
                yield (cdi_mod, cdj_mod), (1, 0)
            elif cdj < 0:
                yield (cdi_mod, cdj_mod), (0, -1)
            elif cdj >= side_len:
                yield (cdi_mod, cdj_mod), (0, 1)
            else:
                yield (cdi_mod, cdj_mod), (0, 0)

    neighbours_precomputed: dict[tuple[int, int], Iterable[tuple[tuple[int, int], tuple[int, int]]]] \
        = dict(((i, j), list(neighbours((i, j)))) for i, row in enumerate(grid) for j, _ in enumerate(row))

    def get_next_day() -> Days:
        next_day = deepcopy(initial)
        for curr, maps in current_day.items():
            if not maps:
                continue
            for neighbour, (mdi, mdj) in neighbours_precomputed[curr]:
                # for neighbour, (mdi, mdj) in neighbours(curr):
                next_day_neighbour = {((mi + mdi), (mj + mdj)) for (mi, mj) in maps}
                next_day[neighbour].update(next_day_neighbour)
                # for (mi, mj) in maps:
                # if ((mi + mdi), (mj + mdj)) in next_day[curr]:
                # print(f"add {((mi + mdi), (mj + mdj))} to {neighbour}")
                # next_day[neighbour].add(((mi + mdi), (mj + mdj)))
                # else:
                #     next_day[curr][(mi + mdi), (mj + mdj)] = amount
        return next_day

    def get_score() -> int:
        return sum(len(maps) for maps in current_day.values())

    d = 0
    while d < days_part_1:
        current_day = get_next_day()
        d += 1

    part_1 = get_score()
    yield part_1

    if side_len == 11:
        yield part_1
        # sample is done here
        return

    # 26501365 == 202300 * 131 + 64, where 131 == side_len

    quadratic_points = []
    print(f"score {get_score()} at day {d}")
    for i, days in enumerate([side_len // 2, side_len // 2 + side_len, side_len // 2 + side_len * 2]):
        while d < days:
            print(f"running for day {d + 1}")
            current_day = get_next_day()
            d += 1
        print(f"score {get_score()} at day {d}")
        quadratic_points += [(i, get_score())]

    def evaluate_quadratic_equation(x):
        # Fit a quadratic polynomial (degree=2) through the points
        coefficients = np.polyfit(*zip(*quadratic_points), 2)

        # Evaluate the quadratic equation at the given x value
        result = np.polyval(coefficients, x)
        return round(result)

    print(quadratic_points)
    yield evaluate_quadratic_equation(202300)


run(solve)
