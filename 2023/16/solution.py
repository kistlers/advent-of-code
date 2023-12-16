from itertools import product
from typing import Iterable

from utils import run


def solve(input_data: str) -> Iterable[int]:
    grid = input_data.splitlines()
    n_rows, n_cols = len(grid), len(grid[0])

    def find_energized(initial: tuple[tuple[int, int], tuple[int, int]]) -> int:
        rays: list[list[dict[tuple[int, int], bool]]] = [
            [dict(zip([(-1, 0), (0, 1), (1, 0), (0, -1)], [False] * 4)) for _ in row] for row in grid]

        stack = [initial]

        def append_stack(next_ij: tuple[int, int], dir_ij: tuple[int, int]) -> None:
            ni, nj = next_ij
            if 0 <= ni < n_rows and 0 <= nj < n_cols and not rays[ni][nj][dir_ij]:
                stack.append((dir_ij, next_ij))

        while stack:
            (di, dj), (current_i, current_j) = stack.pop()
            if rays[current_i][current_j][(di, dj)]:
                continue
            rays[current_i][current_j][(di, dj)] = True

            if grid[current_i][current_j] == '/':
                if di == 0:
                    # reflect up from left and down from right
                    append_stack((current_i - dj, current_j), (-dj, 0))
                if dj == 0:
                    # reflect left from up and right from down
                    append_stack((current_i, current_j - di), (0, -di))

            elif grid[current_i][current_j] == '\\':
                if di == 0:
                    # reflect down from left and up from right
                    append_stack((current_i + dj, current_j), (dj, 0))
                if dj == 0:
                    # reflect left from down and right from up
                    append_stack((current_i, current_j + di), (0, di))


            elif grid[current_i][current_j] == '-' and dj == 0:
                # split left
                append_stack((current_i, current_j - 1), (0, -1))
                # split right
                append_stack((current_i, current_j + 1), (0, 1))

            elif grid[current_i][current_j] == '|' and di == 0:
                # split up
                append_stack((current_i - 1, current_j), (-1, 0))
                # split down
                append_stack((current_i + 1, current_j), (1, 0))

            else:
                append_stack((current_i + di, current_j + dj), (di, dj))

        return sum(any(cell.values()) for row in rays for cell in row)

    initials = (
            [((0, 1), (i, 0)) for i in range(n_rows)] +
            [((0, -1), (i, n_cols - 1)) for i in range(n_rows)] +
            [((1, 0), (0, j)) for j in range(n_cols)] +
            [((-1, 0), (n_rows - 1, j)) for j in range(n_cols)]
    )
    yield find_energized(initials[0])
    yield max(find_energized(initial) for initial in initials)


run(solve)
