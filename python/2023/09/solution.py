import itertools
from typing import Iterable

from python.utils import run


def find_before_after_values(history: list[int]) -> (int, int):
    histories = [history]
    for i in itertools.count():
        current = histories[i]
        next_history = [t - s for s, t in zip(current, current[1:])]
        if all(v == 0 for v in next_history):
            break
        histories += [next_history]

    current_after = 0
    current_before = 0
    for history in reversed(histories):
        current_before = history[0] - current_before
        current_after = current_after + history[-1]

    return current_before, current_after


def solve(input_data: str) -> Iterable[int]:
    input_histories = [list(map(lambda v: int(v), line.split())) for line in input_data.splitlines()]

    before_after_values = [find_before_after_values(history) for history in input_histories]
    yield sum([after for _, after in before_after_values])
    yield sum([before for before, _ in before_after_values])


run(solve)
