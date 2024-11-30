from portion import Interval, OPEN, CLOSED, inf


# https://github.com/AlexandreDecan/portion/issues/24#issuecomment-604456362
def discretize_interval(i: Interval, incr: int = 1) -> Interval:
    def first_step(s: Interval) -> Interval:
        return Interval.from_atomic(
            OPEN,
            (s.lower - incr if s.left is CLOSED else s.lower),
            (s.upper + incr if s.right is CLOSED else s.upper),
            OPEN,
        )

    def second_step(s: Interval) -> Interval:
        return Interval.from_atomic(
            CLOSED,
            (s.lower + incr if s.left is OPEN and s.lower != -inf else s.lower),
            (s.upper - incr if s.right is OPEN and s.upper != inf else s.upper),
            CLOSED,
        )

    return i.apply(first_step).apply(second_step)


def offset_interval(interval: Interval, offset: int) -> Interval:
    def apply_offset(original: Interval) -> Interval:
        return Interval.from_atomic(
            original.left,
            original.lower + offset,
            original.upper + offset,
            original.right,
        )

    return interval.apply(apply_offset)