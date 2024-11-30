import functools
from typing import Iterable

from python.utils import run


@functools.cache
def count_arrangements(conditions, sections):
    if not sections:
        if '#' not in conditions:
            return 1
        return 0
    curr_section, sections = sections[0], sections[1:]
    result = 0
    for i in range(len(conditions) - (curr_section + sum(sections)) - (len(sections) - 1)):
        if "#" in conditions[:i]:
            break
        next_section_start = curr_section + i
        if (next_section_start <= len(conditions)
                and '.' not in conditions[i: next_section_start]
                and conditions[next_section_start: next_section_start + 1] != "#"):
            result += count_arrangements(conditions[next_section_start + 1:], sections)
    return result


def solve(input_data: str) -> Iterable[int]:
    lines = [line.split() for line in input_data.splitlines()]

    arrangements_1 = 0
    arrangements_2 = 0
    for conditions, sections in lines:
        sections = tuple(map(int, sections.split(",")))
        arrangements_1 += count_arrangements(conditions, sections)
        conditions_2 = "?".join([conditions] * 5)
        sections_2 = sections * 5
        arrangements_2 += count_arrangements(conditions_2, sections_2)

    yield arrangements_1
    yield arrangements_2


run(solve)
