from typing import Iterable, Callable, Optional

from utils import run

Pair = tuple[int, int]
Edge = tuple[Pair, int]
Graph = dict[Pair, list[Edge]]


def solve(input_data: str) -> Iterable[int]:
    grid = input_data.splitlines()
    n_rows, n_cols = len(grid), len(grid[0])

    start = [(0, j) for j, cell in enumerate(grid[0]) if cell == '.'][0]
    end = [(n_rows - 1, j) for j, cell in enumerate(grid[n_rows - 1]) if cell == '.'][0]

    def compute_graph(neighbours: Callable[[Pair], Iterable[Pair]]) -> Graph:

        def find_next_crossroads(curr_node: Pair, curr_neighbour: Pair) -> Optional[tuple[Edge, list[Pair]]]:
            visited = {curr_node}
            stack = [(curr_neighbour, 1)]
            while stack:
                curr, d = stack.pop()
                if curr in visited:
                    continue
                visited.add(curr)
                if curr == end:
                    return (curr, d), []
                neighs = [n for n in neighbours(curr) if n not in visited]
                if len(neighs) > 1:
                    return (curr, d), neighs
                if len(neighs) == 1:
                    stack.append((neighs[0], d + 1))
            return None

        graph: dict[Pair, list[Edge]] = {start: [], end: []}

        visited: set[tuple[Pair, Pair]] = set()
        node_stack: list[tuple[Pair, Pair]] = [(start, (start[0] + 1, start[1]))]
        while node_stack:
            curr_node, curr_neighbour = node_stack.pop()
            if (curr_node, curr_neighbour) in visited:
                continue
            visited.add((curr_node, curr_neighbour))

            next_crossroads = find_next_crossroads(curr_node, curr_neighbour)
            if next_crossroads is None:
                continue

            next_edge, next_neighbours = next_crossroads
            if curr_node in graph:
                graph[curr_node].append(next_edge)
            else:
                graph[curr_node] = [next_edge]

            next_node, _ = next_edge
            for nn in next_neighbours:
                node_stack.append((next_node, nn))

        return graph

    def find_longest_path_graph(graph: Graph) -> int:
        distance = dict((n, -1) for n in graph.keys())
        path = dict((n, []) for n in graph.keys())
        stack = [(start, [], 0)]
        while stack:
            curr, current_visited, curr_dist = stack.pop()
            if curr_dist > distance[curr]:
                distance[curr] = curr_dist
                path[curr] = current_visited
            new_neighbours = [
                (neigh, current_visited + [curr], curr_dist + neigh_dist)
                for neigh, neigh_dist in graph[curr] if neigh not in current_visited
            ]
            stack.extend(new_neighbours)

        return distance[end]

    def neighbours_without_uphill(curr: Pair) -> Iterable[Pair]:
        ci, cj = curr
        for (di, dj), allowed_slope in [((-1, 0), '^'), ((0, 1), '>'), ((1, 0), 'v'), ((0, -1), '<')]:
            if 0 <= ci + di < n_rows and 0 <= cj + dj < n_cols and grid[ci + di][cj + dj] in ['.', allowed_slope]:
                yield ci + di, cj + dj

    def neighbours_with_uphill(curr: Pair) -> Iterable[Pair]:
        ci, cj = curr
        for di, dj in [(-1, 0), (0, 1), (1, 0), (0, -1)]:
            if 0 <= ci + di < n_rows and 0 <= cj + dj < n_cols and grid[ci + di][cj + dj] != '#':
                yield ci + di, cj + dj

    yield find_longest_path_graph(compute_graph(neighbours_without_uphill))
    yield find_longest_path_graph(compute_graph(neighbours_with_uphill))


run(solve)
