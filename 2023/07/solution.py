from itertools import groupby
from typing import Iterable

from utils import run
from enum import Enum
from collections import namedtuple

CardValue = namedtuple('CardValue', ['card_value', 'card_value_part_2', 'card'])


class Card(Enum):

    @property
    def card_value(self):
        return self.value.card_value

    @property
    def card_value_part_2(self):
        return self.value.card_value_part_2

    @property
    def card(self):
        return self.value.card

    TWO = CardValue(2, 2, '2')
    THREE = CardValue(3, 3, '3')
    FOUR = CardValue(4, 4, '4')
    FIVE = CardValue(5, 5, '5')
    SIX = CardValue(6, 6, '6')
    SEVEN = CardValue(7, 7, '7')
    EIGHT = CardValue(8, 8, '8')
    NINE = CardValue(9, 9, '9')
    TEN = CardValue(10, 10, 'T')
    J = CardValue(11, 1, 'J')
    Q = CardValue(12, 12, 'Q')
    K = CardValue(13, 13, 'K')
    A = CardValue(14, 14, 'A')


class HandType(Enum):
    HIGH_CARD = 1
    ONE_PAIR = 2
    TWO_PAIR = 3
    THREE_OF_A_KIND = 4
    FULL_HOUSE = 5
    FOUR_OF_A_KIND = 6
    FIVE_OF_A_KIND = 7


Hand = (int, list[int], int)  # hand_type, card_values, bid


def parse_card(card: str) -> Card:
    match card:
        case 'A':
            return Card.A
        case 'K':
            return Card.K
        case 'Q':
            return Card.Q
        case 'J':
            return Card.J
        case 'T':
            return Card.TEN
        case '9':
            return Card.NINE
        case '8':
            return Card.EIGHT
        case '7':
            return Card.SEVEN
        case '6':
            return Card.SIX
        case '5':
            return Card.FIVE
        case '4':
            return Card.FOUR
        case '3':
            return Card.THREE
        case '2':
            return Card.TWO


def find_hand_type(cards_raw: str) -> int:
    cards = [parse_card(card) for card in cards_raw]
    grouped_cards = [(len([v for v in group]), card.card_value) for card, group in
                     groupby(sorted(cards, key=lambda c: c.card_value, reverse=True))]
    grouped_lens_sorted = sorted([v for v, _ in grouped_cards], reverse=True)
    match grouped_lens_sorted:
        case [5]:
            return HandType.FIVE_OF_A_KIND.value
        case [4, 1]:
            return HandType.FOUR_OF_A_KIND.value
        case [3, 1, 1]:
            return HandType.THREE_OF_A_KIND.value
        case [3, 2]:
            return HandType.FULL_HOUSE.value
        case [2, 2, 1]:
            return HandType.TWO_PAIR.value
        case [2, 1, 1, 1]:
            return HandType.ONE_PAIR.value
        case _:
            return HandType.HIGH_CARD.value


def find_hand_type_part_2(cards_raw: str) -> int:
    cards_no_js = [parse_card(card) for card in cards_raw.replace('J', '')]
    grouped_cards_no_js = [(len([v for v in group]), card) for card, group in
                           groupby(sorted(cards_no_js, key=lambda c: c.card_value, reverse=True))]
    grouped_cards_sorted_no_js = sorted(grouped_cards_no_js, key=lambda p: (p[0], p[1].card_value), reverse=True)
    grouped_lens_sorted_no_js = sorted([v for v, _ in grouped_cards_sorted_no_js], reverse=True)
    match grouped_lens_sorted_no_js:
        case [4]:
            j_replacement = grouped_cards_sorted_no_js[0][1].card
            return find_hand_type(cards_raw.replace('J', j_replacement))
        case [3, 1]:
            j_replacement = grouped_cards_sorted_no_js[0][1].card
            return find_hand_type(cards_raw.replace('J', j_replacement))
        case [2, 2]:
            j_replacement = grouped_cards_sorted_no_js[0][1].card
            return find_hand_type(cards_raw.replace('J', j_replacement))
        case [2, 1, 1]:
            j_replacement = grouped_cards_sorted_no_js[0][1].card
            return find_hand_type(cards_raw.replace('J', j_replacement))
        case [1, 1, 1, 1]:
            j_replacement = grouped_cards_sorted_no_js[0][1].card
            return find_hand_type(cards_raw.replace('J', j_replacement))
        case [3]:
            j_replacement = grouped_cards_sorted_no_js[0][1].card
            return find_hand_type(cards_raw.replace('J', j_replacement))
        case [2, 1]:
            j_replacement = grouped_cards_sorted_no_js[0][1].card
            return find_hand_type(cards_raw.replace('J', j_replacement))
        case [1, 1, 1]:
            j_replacement = grouped_cards_sorted_no_js[0][1].card
            return find_hand_type(cards_raw.replace('J', j_replacement))
        case [2]:
            j_replacement = grouped_cards_sorted_no_js[0][1].card
            return find_hand_type(cards_raw.replace('J', j_replacement))
        case [1, 1]:
            j_replacement = grouped_cards_sorted_no_js[0][1].card
            return find_hand_type(cards_raw.replace('J', j_replacement))
        case [1]:
            j_replacement = grouped_cards_sorted_no_js[0][1].card
            return find_hand_type(cards_raw.replace('J', j_replacement))
        case []:
            return find_hand_type('AAAAA')
        case _:
            return find_hand_type(cards_raw)


def parse_hand(cards_raw: str, bid: int) -> Hand:
    hand_type = find_hand_type(cards_raw)
    cards = [parse_card(card) for card in cards_raw]
    card_values = [card.card_value for card in cards]
    return hand_type, card_values, bid


def parse_hand_part_2(cards_raw: str, bid: int) -> Hand:
    cards = [parse_card(card) for card in cards_raw]
    hand_type = find_hand_type_part_2(cards_raw)
    card_values = [card.card_value_part_2 for card in cards]
    return hand_type, card_values, bid


def solve_hands(hands: list[Hand]) -> int:
    sorted_hands = sorted(hands)
    sorted_bids = [hand[2] * i for hand, i in zip(sorted_hands, range(1, len(hands) + 1))]
    return sum(sorted_bids)


def solve(input_data: str) -> Iterable[int]:
    lines = input_data.splitlines()
    lines_split = [line.split() for line in lines]

    hands_part_1 = [parse_hand(line_split[0], int(line_split[1])) for line_split in lines_split]
    yield solve_hands(hands_part_1)

    hands_part_2 = [parse_hand_part_2(line_split[0], int(line_split[1])) for line_split in lines_split]
    yield solve_hands(hands_part_2)


run(solve)
