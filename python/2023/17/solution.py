import sys
from itertools import product
from typing import Iterable
from astar import AStar

from python.utils import run

Node = tuple[tuple[int, int], tuple[int, int]]


class Aoc17AStar(AStar):
    def __init__(self, nodes: dict[Node, list[tuple[Node, int]]]):
        self.nodes = nodes

    def neighbors(self, n) -> Iterable[Node]:
        curr, curr_dir = n
        for (n1, n1_dir), d in self.nodes[n]:
            if curr_dir != n1_dir:
                yield n1, n1_dir

    def distance_between(self, n1: Node, n2: Node) -> int:
        for n, d in self.nodes[n1]:
            if n == n2:
                return d
        return sys.maxsize

    def heuristic_cost_estimate(self, current: Node, goal: Node) -> int:
        """
        Manhattan distance between current and goal
        """
        (ci, cj), _ = current
        ((gi, gj), _) = goal
        return abs(ci - gi) + abs(cj - gj)

    def is_goal_reached(self, current: Node, goal: Node):
        return current == goal


def solve(input_data: str) -> Iterable[int]:
    heat_loss = [[int(cell) for cell in row] for row in input_data.splitlines()]
    n_rows, n_cols = len(heat_loss), len(heat_loss[0])
    directions: list[tuple[int, int]] = [(0, -1), (1, 0), (0, 1), (-1, 0)]
    next_directions: dict[tuple[int, int], list[tuple[int, int]]] = {
        (0, -1): [(1, 0), (-1, 0)],
        (0, 1): [(1, 0), (-1, 0)],
        (-1, 0): [(0, 1), (0, -1)],
        (1, 0): [(0, 1), (0, -1)]
    }

    def build_graph(next_distances_min: int, next_distances_max: int) -> Aoc17AStar:
        nodes: dict[Node, list[tuple[Node, int]]] = dict()

        distances_summing_range = list(range(1, next_distances_max + 1))
        next_distances = list(range(next_distances_min, next_distances_max + 1))

        nodes[((0, 0), (0, 0))] = [(((0, 0), d), 0) for d in directions]
        for i, j in product(range(n_rows), range(n_cols)):
            curr = i, j
            for curr_dir in directions:
                di, dj = curr_dir
                curr_neighbours: list[tuple[Node, int]] = []
                heat_loss_sum: int = 0
                for dist in distances_summing_range:
                    neighbour = ni, nj = i + di * dist, j + dj * dist
                    if (0 > ni or ni >= n_rows) or (0 > nj or nj >= n_cols):
                        break
                    heat_loss_sum += heat_loss[ni][nj]
                    if dist not in next_distances:
                        continue
                    for neighbour_dir in next_directions[curr_dir]:
                        curr_neighbours.append(((neighbour, neighbour_dir), heat_loss_sum))
                nodes[(curr, curr_dir)] = curr_neighbours

        for d in directions:
            nodes[((n_rows - 1, n_cols - 1), d)] += [(((n_rows - 1, n_cols - 1), (0, 0)), 0)]

        return Aoc17AStar(nodes)

    source = ((0, 0), (0, 0))
    target = ((n_rows - 1, n_cols - 1), (0, 0))

    def solve_part(next_distances_min: int, next_distances_max: int) -> int:
        graph = build_graph(next_distances_min, next_distances_max)
        path = tuple(graph.astar(source, target))[1:-1]
        distances = [graph.distance_between(a, b) for a, b in zip(path, path[1:])]

        return sum(distances)

    yield solve_part(1, 3)
    yield solve_part(4, 10)


run(solve)
