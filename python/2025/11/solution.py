import sys
from typing import Iterable

sys.path.append('/opt/homebrew/opt/graph-tool/lib/python3.14/site-packages/graph_tool')

# noinspection PyUnresolvedReferences
import graph_tool.all as gt  # brew install graph-tool
from python.utils import run


def solve(input_data: str) -> Iterable[int]:
    edges = [(line.split(": ")[0], b, 1) for line in input_data.strip().splitlines() for b in
             line.split(": ")[1].split()]
    g = gt.Graph(edges, eprops=[('weight', 'double')], hashed=True, directed=False)
    weight = g.edge_properties["weight"]
    mc, part = gt.min_cut(g, weight)

    partition_a = sum(1 for p in part if p)
    partition_b = sum(1 for p in part if not p)
    yield partition_a * partition_b


run(solve)
