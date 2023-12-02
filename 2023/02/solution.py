from dataclasses import dataclass
from typing import Iterable

from utils import run
import functools


@dataclass
class Handful:
    red: int
    green: int
    blue: int

    def is_valid(self):
        return self.red <= 12 and self.green <= 13 and self.blue <= 14

    def power(self):
        return self.red * self.green * self.blue


def parse_handful(handful_string: str) -> Handful:
    red = 0
    green = 0
    blue = 0

    for count_color in handful_string.split(", "):
        match (count_color.split(" ")):
            case [d, "red"]:
                red = int(d)
            case [d, "green"]:
                green = int(d)
            case [d, "blue"]:
                blue = int(d)

    return Handful(red, green, blue)


def parse_handfuls(handfuls_string: str) -> list[Handful]:
    return [parse_handful(s) for s in handfuls_string.split("; ")]


def is_valid_part_1(handfuls: list[Handful]):
    for handful in handfuls:
        if not handful.is_valid():
            return False

    return True


def find_minimum_marbles(handfuls: list[Handful]):
    return functools.reduce(
        lambda a, b: Handful(
            max(a.red, b.red),
            max(a.green, b.green),
            max(a.blue, b.blue)
        ), handfuls)


def solve(input_data: str) -> Iterable[int]:
    lines = input_data.splitlines()
    part_1 = 0
    part_2 = 0
    for line in lines:
        game_string, *handfuls_string = line.split(": ", 2)
        handfuls = parse_handfuls(handfuls_string[0])

        game_id = int(game_string.split(" ")[1])
        if is_valid_part_1(handfuls):
            part_1 += game_id

        part_2 += find_minimum_marbles(handfuls).power()

    yield part_1
    yield part_2


run(solve)
