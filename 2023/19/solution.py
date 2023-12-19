from dataclasses import dataclass, astuple
from typing import Iterable, Callable

from utils import run


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


Condition = Callable[[Part], tuple[bool, str]]


def parse_condition(condition_string: str) -> Condition:
    if ":" not in condition_string:
        return lambda part: (True, condition_string)

    condition, next_workflow = condition_string.split(":")

    if "=" in condition_string:
        rating_name, condition_value = condition.split("=")
        return lambda p: (True, next_workflow) if p.get_rating(rating_name) == int(condition_value) else (False, None)

    if "<" in condition_string:
        rating_name, condition_value = condition.split("<")
        return lambda p: (True, next_workflow) if p.get_rating(rating_name) < int(condition_value) else (False, None)

    rating_name, condition_value = condition.split(">")
    return lambda p: (True, next_workflow) if p.get_rating(rating_name) > int(condition_value) else (False, None)


@dataclass
class Workflow:
    name: str
    conditions: list[Condition]

    def __iter__(self):
        return iter(astuple(self))

    def process_part(self, part: Part) -> str:
        for condition in self.conditions:
            accepted, next_workflow = condition(part)
            if accepted:
                return next_workflow
        raise Exception(f"should not be here, processed all conditions")


def parse_workflow(part_workflow: str) -> Workflow:
    name, conditions_string = part_workflow.split("{")
    conditions = [parse_condition(condition_string) for condition_string in conditions_string[:-1].split(",")]
    return Workflow(name, conditions)


def solve(input_data: str) -> Iterable[int]:
    workflow_lines, part_lines = input_data.split("\n\n")
    workflows = {workflow.name: workflow
                 for workflow
                 in [parse_workflow(workflow_line) for workflow_line in workflow_lines.splitlines()]}
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

    rating_sum = 0
    for part in parts:
        if part_is_accepted(part):
            rating_sum += part.rating_sum()

    yield rating_sum


run(solve)
