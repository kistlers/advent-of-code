from typing import Iterable

from python.utils import run
import re


def string_to_digit(digit: str) -> str:
    match digit:
        case 'one':
            return '1'
        case 'two':
            return '2'
        case 'three':
            return '3'
        case 'four':
            return '4'
        case 'five':
            return '5'
        case 'six':
            return '6'
        case 'seven':
            return '7'
        case 'eight':
            return '8'
        case 'nine':
            return '9'
        case digit:
            return digit


def solve(input_data: str) -> Iterable[int]:
    lines = input_data.splitlines()

    calibration_sum_1 = 0
    calibration_sum_2 = 0

    for line in lines:
        digits_1 = [s for s in list(line) if s.isdigit()]
        matches_iter = re.finditer(r'(?=(one|two|three|four|five|six|seven|eight|nine|\d))', line)
        matches = [match.group(1) for match in matches_iter]
        digits_2 = list(map(string_to_digit, matches))
        calibration_sum_1 += int(digits_1[0] + '' + digits_1[-1])
        calibration_sum_2 += int(digits_2[0] + '' + digits_2[-1])

    yield calibration_sum_1
    yield calibration_sum_2


run(solve)
