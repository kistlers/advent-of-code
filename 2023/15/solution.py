from functools import reduce
from typing import Iterable

from utils import run


def hash_sequence(sequence: str) -> int:
    return reduce(lambda result, char: (result + ord(char)) * 17 % 256, sequence, 0)


def solve(input_data: str) -> Iterable[int]:
    sequences = input_data.splitlines()[0].split(",")

    hashed_sequences = [hash_sequence(sequence) for sequence in sequences]
    yield sum(hashed_sequences)

    boxes: list[list[tuple[str, int]]] = [[] for _ in range(256)]
    label_indices: list[dict[str, int]] = [{} for _ in range(256)]
    for sequence in sequences:
        if '=' in sequence:
            label, focal_length = sequence.split('=')
            focal_length = int(focal_length)
            hashed = hash_sequence(label)
            if label in label_indices[hashed]:
                label_index = label_indices[hashed][label]
                boxes[hashed][label_index] = (label, focal_length)
            else:
                new_label_index = len(boxes[hashed])
                boxes[hashed].append((label, focal_length))
                label_indices[hashed][label] = new_label_index
        if '-' in sequence:
            label = sequence.split('-')[0]
            hashed = hash_sequence(label)
            if label in label_indices[hashed]:
                label_index = label_indices[hashed][label]
                boxes[hashed].pop(label_index)
                label_indices[hashed].pop(label)
                for label_index_hashed_key in label_indices[hashed].keys():
                    current_label_index = label_indices[hashed][label_index_hashed_key]
                    if current_label_index > label_index:
                        label_indices[hashed][label_index_hashed_key] = current_label_index - 1

    focusing_powers = [
        (i + 1) * (j + 1) * focal_length
        for i, box in enumerate(boxes)
        for j, (label, focal_length) in enumerate(box)
    ]
    yield sum(focusing_powers)


run(solve)
