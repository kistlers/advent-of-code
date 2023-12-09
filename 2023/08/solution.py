import itertools
from typing import Iterable

from utils import run
import re


def solve(input_data: str) -> Iterable[int]:
    lines = input_data.splitlines()

    neighbours = dict()
    for line in lines[2:]:
        node, left, right = [match.group(1) for match in re.finditer(r'(\w+)', line)]
        print(f"{node} -> {left}, {right}")
        neighbours[node] = (left, right)

    print(neighbours)

    current = 'AAA'
    target = 'ZZZ'
    steps = 0
    for i, instruction in enumerate(itertools.cycle(list(lines[0]))):
        if current == target:
            steps = i
            break
        elif instruction == 'L':
            current = neighbours[current][0]
        elif instruction == 'R':
            current = neighbours[current][1]

        pass

    yield steps


run(solve)
