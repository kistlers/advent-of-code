import math
from collections import deque
from typing import Iterable, Optional

from python.utils import run


class Module:
    name: str
    output_modules: [str]

    def __init__(self, name: str, output_modules: list[str]):
        self.name = name
        self.output_modules = output_modules

    def pulse(self, pulse_from: str, input_pulse: bool) -> Optional[bool]:
        return input_pulse

    def __repr__(self):
        return f"Module(name={self.name}, output_modules={self.output_modules})"


class FlipFlop(Module):
    state: bool

    def __init__(self, name: str, output_modules: list[str], state: bool):
        super().__init__(name, output_modules)
        self.state = state

    def __repr__(self):
        return f"FlipFlop(name={self.name}, output_modules={self.output_modules}, state={self.state})"

    def pulse(self, pulse_from: str, input_pulse: bool) -> Optional[bool]:
        if input_pulse:
            # ignore high pulse
            return None

        self.state = not self.state
        return self.state


class Conjunction(Module):
    states: dict[str, bool]

    def __init__(self, name: str, output_modules: list[str], states: dict[str, bool]):
        super().__init__(name, output_modules)
        self.states = states

    def __repr__(self):
        return f"Conjunction(name={self.name}, output_modules={self.output_modules}, states={self.states})"

    def pulse(self, pulse_from: str, input_pulse: bool) -> Optional[bool]:
        self.states[pulse_from] = input_pulse
        return not all(state for state in self.states.values())


def parse_module(module_line: str) -> tuple[str, Module]:
    module_raw, outputs_raw = module_line.split(" -> ")
    outputs = outputs_raw.split(", ")
    if module_raw.startswith("%"):
        return module_raw[1:], FlipFlop(module_raw[1:], outputs, False)
    if module_raw.startswith("&"):
        return module_raw[1:], Conjunction(module_raw[1:], outputs, dict())

    return module_raw, Module(module_raw, outputs)


def solve(input_data: str) -> Iterable[int]:
    modules = dict(parse_module(module_line) for module_line in input_data.splitlines())
    for module in modules.values():
        for output_module in module.output_modules:
            if output_module in modules and isinstance(modules[output_module], Conjunction):
                # noinspection PyUnresolvedReferences
                modules[output_module].states[module.name] = False

    low_pulses = 0
    high_pulses = 0
    rx_received_low: Optional[int] = None
    if len(input_data.splitlines()) < 50:
        # not rx for sample input
        rx_received_low = 1

    # "rx" has only one input "bn"
    # "bn" has four inputs "pl", "lz", "mz" and "zm"
    pl_sends_high: Optional[int] = None
    lz_sends_high: Optional[int] = None
    mz_sends_high: Optional[int] = None
    zm_sends_high: Optional[int] = None

    def simulate_button_push(button_push: int):
        nonlocal low_pulses, high_pulses
        nonlocal pl_sends_high, lz_sends_high, mz_sends_high, zm_sends_high

        low_pulses += 1
        queue: deque[tuple[str, bool, str]] = deque([("button", False, "broadcaster")])
        while queue:
            pulse_from, input_pulse, module_name = queue.popleft()

            if module_name not in modules:
                continue

            current_module = modules[module_name]
            next_pulse = current_module.pulse(pulse_from, input_pulse)

            if "pl" == module_name and next_pulse and not pl_sends_high:
                pl_sends_high = button_push
            if "lz" == module_name and next_pulse and not lz_sends_high:
                lz_sends_high = button_push
            if "mz" == module_name and next_pulse and not mz_sends_high:
                mz_sends_high = button_push
            if "zm" == module_name and next_pulse and not zm_sends_high:
                zm_sends_high = button_push

            if next_pulse is None:
                continue

            if next_pulse:
                high_pulses += len(current_module.output_modules)
            else:
                low_pulses += len(current_module.output_modules)

            next_pulses = [
                (module_name, next_pulse, output_module_name)
                for output_module_name in current_module.output_modules
            ]
            queue.extend(next_pulses)

    i = 1
    while i <= 1000 or rx_received_low is None:
        simulate_button_push(i)
        if i == 1000:
            yield low_pulses * high_pulses

        if pl_sends_high and lz_sends_high and mz_sends_high and zm_sends_high:
            rx_received_low = math.lcm(pl_sends_high, lz_sends_high, mz_sends_high, zm_sends_high)

        i += 1

    yield rx_received_low


run(solve)
