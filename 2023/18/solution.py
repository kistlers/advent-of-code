from typing import Iterable

from utils import run


def solve(input_data: str) -> Iterable[int]:
    lines = [(line.split()[0], int(line.split()[1]), line.split()[2][1:-1]) for line in input_data.splitlines()]
    print(lines)

    current = ci, cj = 0, 0
    trench = {current}

    for new_direction, amount, _ in lines:
        match new_direction:
            case 'R':
                trench = trench.union({(ci, nj) for nj in range(cj + 1, cj + 1 + amount)})
                cj += amount
            case 'L':
                trench = trench.union({(ci, nj) for nj in range(cj - amount, cj)})
                cj -= amount
            case 'D':
                trench = trench.union({(ni, cj) for ni in range(ci + 1, ci + 1 + amount)})
                ci += amount
            case 'U':
                trench = trench.union({(ni, cj) for ni in range(ci - amount, ci)})
                ci -= amount
            case _:
                raise Exception('Invalid direction')

    max_i = max(ti for ti, _ in trench)
    min_i = min(ti for ti, _ in trench)
    max_j = max(tj for _, tj in trench)
    min_j = min(tj for _, tj in trench)

    start = (min_i - 1, min_j - 1)
    outside = []
    stack = [start]
    while stack:
        current = ci, cj = stack.pop()
        if current in outside or current in trench:
            continue
        outside.append(current)
        for ni, nj in [(ci - 1, cj), (ci + 1, cj), (ci, cj - 1), (ci, cj + 1)]:
            pass
            if (min_i - 1 <= ni <= max_i + 1
                    and min_j - 1 <= nj <= max_j + 1
                    and (ni, nj) not in outside
                    and (ni, nj) not in trench):
                stack.append((ni, nj))

    outside_size = len(outside)
    all_size = (abs(max_i + 1 - min_i) + 2) * (abs(max_j + 1 - min_j) + 2)
    yield all_size - outside_size


run(solve)
