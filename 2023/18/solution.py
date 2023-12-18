from typing import Iterable

from utils import run


def find_corners(instructions: list[tuple[str, int]]) -> tuple[list[tuple[int, int]], int]:
    current = ci, cj = 0, 0
    trench = [current]
    current_dir = instructions[-1][0]  # last direction == first direction
    turns = []

    def get_turn(new_dir: str) -> str:
        if (current_dir, new_dir) in [('U', 'R'), ('R', 'D'), ('D', 'L'), ('L', 'U')]:
            return 'R'
        if (current_dir, new_dir) in [('U', 'L'), ('L', 'D'), ('D', 'R'), ('R', 'U')]:
            return 'L'
        raise Exception("should not be here")

    for new_direction, amount in instructions:
        match new_direction:
            case 'R':
                cj += amount
            case 'L':
                cj -= amount
            case 'D':
                ci += amount
            case 'U':
                ci -= amount
            case _:
                raise Exception('Invalid direction')
        trench += [(ci, cj)]
        turns += [get_turn(new_direction)]
        current_dir = new_direction

    count_rs = len([t for t in turns if t == 'R'])
    count_ls = len([t for t in turns if t == 'L'])
    if abs(count_rs - count_ls) != 4:
        raise Exception(
            f"difference between right and left turns should be exactly 4: count_rs: {count_rs}, count_ls: {count_ls}")

    if count_ls > count_rs:
        # 4 quarters are counted doubled on left-turning trench
        return trench, -1

    # trench must be in Counter-clock-wise order for shoelace formula and
    # 4 quarters are not counted doubled on right-turning trench
    return [trench[0]] + list(reversed(trench[:-1])), 1


def find_area(trench: list[tuple[int, int]], area_correction: int) -> int:
    """
    https://www.101computing.net/the-shoelace-algorithm/
    """
    trench_size = len(trench)
    sum1 = 0
    sum2 = 0

    for i in range(trench_size - 1):
        sum1 += trench[i][0] * trench[i + 1][1]
        sum2 += trench[i][1] * trench[i + 1][0]

    # Add xn*y1
    sum1 += trench[-1][0] * trench[0][1]
    # Add x1*yn
    sum2 += trench[0][0] * trench[-1][1]

    inner_area = abs(sum1 - sum2) // 2

    trench_length = 0
    for (ai, aj), (bi, bj) in zip(trench, [*trench[1:], trench[0]]):
        trench_length += abs(bi - ai) + abs(bj - aj)
    area = inner_area + trench_length // 2 + area_correction
    return area


def solve_part(instructions: list[tuple[str, int]]) -> int:
    trench, area_correction = find_corners(instructions)
    area = find_area(trench, area_correction)
    return area


def solve(input_data: str) -> Iterable[int]:
    instructions_1 = []
    instructions_2 = []
    for line in input_data.splitlines():
        dir_1, amount, color = line.split()
        match int(color[-2]):
            case 0:
                dir_2 = 'R'
            case 1:
                dir_2 = 'D'
            case 2:
                dir_2 = 'L'
            case 3:
                dir_2 = 'U'
            case _:
                raise Exception("Unknown direction")
        instructions_1.append((dir_1, int(amount)))
        instructions_2.append((dir_2, int(color[2:7], base=16)))

    yield solve_part(instructions_1)
    yield solve_part(instructions_2)


run(solve)
