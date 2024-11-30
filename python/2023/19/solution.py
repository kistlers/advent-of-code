import math
from dataclasses import dataclass
from enum import Enum
from typing import Iterable, Callable

import portion as P

from python.utils import run
from python.utils.interval_utils import discretize_interval

XmasInterval = list[P.Interval, P.Interval, P.Interval, P.Interval]


@dataclass
class Part:
    x: int
    m: int
    a: int
    s: int

    def rating_sum(self) -> int:
        return self.x + self.m + self.a + self.s

    def get_rating(self, rating_name: str):
        match rating_name:
            case "x":
                return self.x
            case "m":
                return self.m
            case "a":
                return self.a
            case "s":
                return self.s
            case r:
                raise Exception(f"should not be here, unknown variable '{r}'")


def parse_part(part_string: str) -> Part:
    x, m, a, s = [int(r.split("=")[1]) for r in part_string[1:-1].split(",")]
    return Part(x, m, a, s)


class Operator(Enum):
    LESS = 1
    GREATER = 2
    NONE = 3


@dataclass
class Condition:
    op: Operator
    variable_index: int
    value: int
    next_workflow_name: str
    evaluate: Callable[[Part], tuple[bool, str]]


def parse_condition(condition_string: str) -> Condition:
    if ":" not in condition_string:
        return Condition(Operator.NONE, -1, -1, condition_string, lambda part: (True, condition_string))

    condition, next_workflow = condition_string.split(":")

    if "<" in condition_string:
        operator = Operator.LESS
        variable_name, condition_value = condition.split("<")
        evaluate = lambda p: (True, next_workflow) if p.get_rating(variable_name) < int(condition_value) else (
            False, None)
    else:
        operator = Operator.GREATER
        variable_name, condition_value = condition.split(">")
        evaluate = lambda p: (True, next_workflow) if p.get_rating(variable_name) > int(condition_value) else (
            False, None)

    match variable_name:
        case "x":
            variable_index = 0
        case "m":
            variable_index = 1
        case "a":
            variable_index = 2
        case "s":
            variable_index = 3
        case v:
            raise Exception(f"should not be here, unknown variable '{v}'")

    return Condition(operator, variable_index, int(condition_value), next_workflow, evaluate)


@dataclass
class Workflow:
    conditions: list[Condition]

    def process_part(self, part: Part) -> str:
        for condition in self.conditions:
            accepted, next_workflow = condition.evaluate(part)
            if accepted:
                return next_workflow
        raise Exception(f"should not be here, processed all conditions")


def parse_workflow(part_workflow: str) -> tuple[str, Workflow]:
    name, conditions_string = part_workflow.split("{")
    conditions = [parse_condition(condition_string) for condition_string in conditions_string[:-1].split(",")]
    return name, Workflow(conditions)


def solve(input_data: str) -> Iterable[int]:
    workflow_lines, part_lines = input_data.split("\n\n")
    workflows = dict([parse_workflow(workflow_line) for workflow_line in workflow_lines.splitlines()])
    parts = [parse_part(part_line) for part_line in part_lines.splitlines()]

    def part_is_accepted(part: Part) -> bool:
        current_workflow = workflows["in"]
        while current_workflow:
            next_workflow_name = current_workflow.process_part(part)
            match next_workflow_name:
                case 'A':
                    return True
                case 'R':
                    return False
            current_workflow = workflows[next_workflow_name]
        else:
            raise Exception(f"should not be here, processed all conditions")

    yield sum(part.rating_sum() for part in parts if part_is_accepted(part))

    def find_valid_options(current_xmas: XmasInterval, current_workflow_name: str) -> int:
        match current_workflow_name:
            case "A":
                return math.prod([interval.upper - interval.lower + 1 for interval in current_xmas])
            case "R":
                return 0
            case _:
                pass

        valid_options = 0
        current_workflow = workflows[current_workflow_name]
        for condition in current_workflow.conditions:
            current_condition_interval = current_xmas[condition.variable_index]
            match condition.op:
                case Operator.LESS:
                    if current_condition_interval < P.singleton(condition.value):
                        valid_options += find_valid_options(current_xmas, condition.next_workflow_name)
                        continue

                    if condition.value in current_condition_interval:
                        new_interval = discretize_interval(
                            current_condition_interval & P.closed(1, condition.value - 1))
                        # noinspection PyTypeChecker
                        new_xmas: XmasInterval = list(current_xmas[:condition.variable_index]) + [new_interval] + list(
                            current_xmas[condition.variable_index + 1:])

                        assert len(new_xmas) == len(current_xmas)
                        valid_options += find_valid_options(new_xmas, condition.next_workflow_name)
                        current_xmas[condition.variable_index] = current_condition_interval & P.closed(condition.value,
                                                                                                       4000)
                        continue

                case Operator.GREATER:
                    if current_condition_interval > P.singleton(condition.value):
                        valid_options += find_valid_options(current_xmas, condition.next_workflow_name)
                        continue

                    if condition.value in current_condition_interval:
                        new_interval = discretize_interval(
                            current_condition_interval & P.closed(condition.value + 1, 4000))
                        # noinspection PyTypeChecker
                        new_xmas: XmasInterval = list(current_xmas[:condition.variable_index]) + [new_interval] + list(
                            current_xmas[condition.variable_index + 1:])
                        assert len(new_xmas) == len(current_xmas)
                        valid_options += find_valid_options(new_xmas, condition.next_workflow_name)
                        current_xmas[condition.variable_index] = current_condition_interval & P.closed(1,
                                                                                                       condition.value)
                        continue

                case Operator.NONE:
                    valid_options += find_valid_options(current_xmas, condition.next_workflow_name)
                case _:
                    raise Exception("invalid operator")

            return valid_options

    # noinspection PyTypeChecker
    initial_intervals: XmasInterval = [P.closed(1, 4000), P.closed(1, 4000), P.closed(1, 4000), P.closed(1, 4000)]
    yield find_valid_options(initial_intervals, "in")


run(solve)
