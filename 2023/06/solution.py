import math
from typing import Iterable

from utils import run


def is_winning_speed(speed: int, time: int, distance: int) -> int:
    travelled = speed * (time - speed)
    return travelled > distance


def calculate_race_wins(time: int, distance: int) -> int:
    min_win_speed = time + 1
    max_win_speed = -1
    # find the lowest winning speed
    for speed in range(time):
        if is_winning_speed(speed, time, distance):
            min_win_speed = speed
            break
    # find the highest winning speed
    for speed in reversed(range(time)):
        if is_winning_speed(speed, time, distance):
            max_win_speed = speed
            break
    # all speeds in [min_win_speed, max_win_speed] are winning speeds
    if max_win_speed > min_win_speed:
        return max_win_speed - min_win_speed + 1

    return 0


def solve(input_data: str) -> Iterable[int]:
    lines = input_data.splitlines()
    times = [int(time) for time in lines[0].split()[1:]]
    distances = [int(distance) for distance in lines[1].split()[1:]]

    part_1_scores = [calculate_race_wins(time, distance) for time, distance in zip(times, distances)]
    part_1 = math.prod(filter(lambda score: score > 0, part_1_scores))
    yield part_1

    part_2_time = int(''.join([str(time) for time in times]))
    part_2_distance = int(''.join([str(distance) for distance in distances]))
    part_2 = calculate_race_wins(part_2_time, part_2_distance)
    yield part_2


run(solve)
