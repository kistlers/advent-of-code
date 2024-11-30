from typing import Iterable

from python.utils import run


def solve_part_1(num_cards: int, lines: list[str]):
    score = 0
    matching_cards = [0] * num_cards

    for line in lines:
        split_line = line.split(": ")
        card_number = int(split_line[0].split()[1])
        numbers = split_line[1].split(" | ")
        winning = set(map(lambda n: int(n), numbers[0].split()))
        have = set(map(lambda n: int(n), numbers[1].split()))
        intersection = winning.intersection(have)
        matching_cards[card_number - 1] = len(intersection)
        if len(intersection) > 0:
            score += pow(2, len(intersection) - 1)

    return score, matching_cards


def solve_part_2(num_cards: int, matching_cards: list[int]):
    total_scratchcards = num_cards

    current_card = 0
    number_of_cards = [1] * num_cards
    while True:
        if all(x == 0 for x in number_of_cards):
            return total_scratchcards

        if number_of_cards[current_card] > 0:
            mc = matching_cards[current_card]
            nc = number_of_cards[current_card]
            for i in range(current_card + 1, current_card + 1 + mc):
                total_scratchcards += nc
                number_of_cards[i] += nc
            number_of_cards[current_card] = 0

        current_card = current_card + 1 % num_cards


def solve(input_data: str) -> Iterable[int]:
    lines = input_data.splitlines()
    num_cards = len(lines)

    part_1, matching_cards = solve_part_1(num_cards, lines)
    part_2 = solve_part_2(num_cards, matching_cards)

    yield part_1
    yield part_2


run(solve)
