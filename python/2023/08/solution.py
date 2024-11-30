import itertools
import math
from typing import Iterable

from python.utils import run
import re


def find_steps(instructions: list[str], neighbours: dict[str, (str, str)], start_node: str,
               end_nodes: list[str]) -> int:
    current = start_node
    for step, instruction in enumerate(itertools.cycle(instructions)):
        if current in end_nodes:
            return step
        elif instruction == 'L':
            current = neighbours[current][0]
        elif instruction == 'R':
            current = neighbours[current][1]
        pass


def solve(input_data: str) -> Iterable[int]:
    lines = input_data.splitlines()

    instructions = list(lines[0])

    neighbours = dict()
    for line in lines[2:]:
        node, left, right = [match.group(1) for match in re.finditer(r'(\w+)', line)]
        print(f"{node} -> {left}, {right}")
        neighbours[node] = (left, right)

    yield find_steps(instructions, neighbours, 'AAA', ['ZZZ'])

    a_nodes = list(filter(lambda n: n.endswith('A'), neighbours.keys()))
    z_nodes = list(filter(lambda n: n.endswith('Z'), neighbours.keys()))
    steps = [find_steps(instructions, neighbours, a_node, z_nodes) for a_node in a_nodes]
    yield math.lcm(*steps)


run(solve)
