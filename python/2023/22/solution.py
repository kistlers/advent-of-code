from typing import Iterable

from python.utils import run

Corner = tuple[int, int, int]
Brick = tuple[Corner, Corner]


def sort_zxy(corners: Iterable[Corner]) -> Iterable[Corner]:
    # sort brick corners by z, x, y component
    return sorted(corners, key=lambda c: (c[-1], *c[:-1]))


def solve(input_data: str) -> Iterable[int]:
    # noinspection PyTypeChecker
    bricks: list[Brick] = sorted(
        [tuple(
            sort_zxy(list(tuple(map(int, b.split(","))) for b in line.split("~")))
        ) for line in input_data.splitlines()],
        # sort bricks by z, x, y component of first corner
        key=lambda b: (b[0][-1], *b[0][:-1])
    )

    occupied: set[Corner] = set()

    def find_new_z(b: Brick) -> int:
        (bx1, by1, bz1), (bx2, by2, bz2) = b
        for z in reversed(range(1, bz1)):
            for x in range(bx1, bx2 + 1):
                for y in range(by1, by2 + 1):
                    if (x, y, z) in occupied:
                        return z + 1
        return 1

    fallen_bricks: list[Brick] = []
    corner_to_fallen_brick_index: dict[Corner, int] = {}

    def occupy(b: Brick, new_z: int, i: int) -> None:
        nonlocal fallen_bricks, corner_to_fallen_brick_index
        (bx1, by1, bz1), (bx2, by2, bz2) = b
        z_shift = bz1 - new_z
        bz1_new, bz2_new = bz1 - z_shift, bz2 - z_shift
        new_occupy: set[Corner] = {
            (x, y, z)
            for x in range(bx1, bx2 + 1)
            for y in range(by1, by2 + 1)
            for z in range(bz1_new, bz2_new + 1)
        }
        corner_to_fallen_brick_index.update(dict(zip(new_occupy, [i] * len(new_occupy))))
        fallen_bricks += [((bx1, by1, bz1_new), (bx2, by2, bz2_new))]
        occupied.update(new_occupy)

    for i, b in enumerate(bricks):
        new_z = find_new_z(b)
        occupy(b, new_z, i)

    brick_is_supported_by: dict[int, set[int]] = dict((i, set()) for i in range(len(fallen_bricks)))
    brick_supports: dict[int, set[int]] = dict((i, set()) for i in range(len(fallen_bricks)))

    for i, ((bx1, by1, bz1), (bx2, by2, bz2)) in enumerate(fallen_bricks):
        face_below_bottom = {(x, y, bz1 - 1) for x in range(bx1, bx2 + 1) for y in range(by1, by2 + 1)}
        supported_by_bricks = {
            corner_to_fallen_brick_index[(x, y, z)]
            for (x, y, z) in face_below_bottom
            if (x, y, z) in corner_to_fallen_brick_index
        }
        brick_is_supported_by[i] = supported_by_bricks
        for sbb in supported_by_bricks:
            brick_supports[sbb].add(i)

    can_be_disintegrated = set()

    for i in range(len(fallen_bricks)):
        if not brick_supports[i] or all(len(brick_is_supported_by[j]) > 1 for j in brick_supports[i]):
            can_be_disintegrated.add(i)

    yield len(can_be_disintegrated)

    part_2_bricks = 0
    for curr_brick in range(len(fallen_bricks)):
        if curr_brick in can_be_disintegrated:
            continue
        current_removed_bricks = {curr_brick}
        stack = list(brick_supports[curr_brick])
        while stack:
            above = stack.pop()
            if brick_is_supported_by[above].difference(current_removed_bricks):
                # still supported, does not fall
                continue
            current_removed_bricks.add(above)
            stack += list(brick_supports[above])

        # -1 to not include curr_brick
        part_2_bricks += len(current_removed_bricks) - 1
    yield part_2_bricks


run(solve)
