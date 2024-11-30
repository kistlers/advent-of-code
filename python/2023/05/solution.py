from dataclasses import dataclass
import itertools
from typing import Iterable

from python.utils import run


@dataclass
class RangeMap:
    ranges: list[(int, int, int)]

    def map(self, index: int) -> int:
        for dest, src, range_len in self.ranges:
            if src <= index < src + range_len:
                return dest + (index - src)
        return index

    def inverse(self, index: int) -> int:
        for dest, src, range_len in self.ranges:
            if dest <= index < dest + range_len:
                return src + (index - dest)
        return index


def build_range_maps(range_inputs: str) -> RangeMap:
    ranges = []
    for range_input in [line.split() for line in range_inputs.splitlines()[1:]]:
        ranges.append([int(x) for x in range_input])

    return RangeMap(ranges)


def is_part2_seed(seed: int, part_2_ranges: list[(int, int)]):
    for range_start, range_len in part_2_ranges:
        if range_start <= seed < range_start + range_len:
            return True
    return False


def solve_part_2(seed_location_inverse, part_2_ranges: list[(int, int)]):
    # go in reverse from 1 up and find the first that matches range
    for loc in itertools.count():
        seed = seed_location_inverse(loc)
        if is_part2_seed(seed, part_2_ranges):
            return loc


def solve(input_data: str) -> Iterable[int]:
    split_input_data = input_data.split("\n\n")

    part_1_seeds = [int(seed) for seed in split_input_data[0].split(": ")[1].split()]

    seedsoil = build_range_maps(split_input_data[1])
    soilfert = build_range_maps(split_input_data[2])
    fertwater = build_range_maps(split_input_data[3])
    waterlight = build_range_maps(split_input_data[4])
    lighttemp = build_range_maps(split_input_data[5])
    temphumid = build_range_maps(split_input_data[6])
    humidloc = build_range_maps(split_input_data[7])

    def seed_location(s: int) -> int:
        return humidloc.map(temphumid.map(lighttemp.map(waterlight.map(fertwater.map(soilfert.map(seedsoil.map(s)))))))

    def seed_location_inverse(s: int) -> int:
        return seedsoil.inverse(soilfert.inverse(
            fertwater.inverse(waterlight.inverse(lighttemp.inverse(temphumid.inverse(humidloc.inverse(s)))))))

    part_1 = min(seed_location(s) for s in part_1_seeds)
    yield part_1

    part_2_ranges = [(range_start, range_len) for range_start, range_len in zip(part_1_seeds[::2], part_1_seeds[1::2])]
    part_2 = solve_part_2(seed_location_inverse, part_2_ranges)
    yield part_2


run(solve)
