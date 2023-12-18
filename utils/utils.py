import json
import os
from copy import deepcopy
from time import time
from typing import Callable, Tuple, Iterable, Optional

import html2text
import requests
from bs4 import BeautifulSoup
from colorama import Fore, Style
from dotenv import load_dotenv

load_dotenv()

CURRENT_DIR = os.getcwd()
HEADERS = {
    "cookie": f"session={os.environ['SESSION_COOKIE']}",
}


def request_content(year: int, day: int, content_type: str) -> str:
    if content_type == "input":
        url = f"https://adventofcode.com/{year}/day/{day}/input"
    elif content_type == "problem.md":
        url = f"https://adventofcode.com/{year}/day/{day}"
    else:
        raise AttributeError(f"Invalid {content_type = }")

    response = requests.get(url, headers=HEADERS)
    handle_error_status(response.status_code)
    return response.text.strip()


def fetch(year: int, day: int, input_file: str) -> str:
    content = request_content(year, day, input_file)
    if input_file == "input":
        return content
    elif input_file == "problem.md":
        soup = BeautifulSoup(content, "html.parser")
        html_to_text = html2text.HTML2Text()
        return "\n\n\n".join(
            [html_to_text.handle(str(a)) for a in soup.select("article")]
        )


def fetch_and_save(year: int, day: int, input_file: str) -> None:
    print(f"🛷 Fetching {input_file} for {day} {year}")
    content = fetch(year, day, input_file)
    with open(f"{CURRENT_DIR}/{input_file}", "w") as text_file:
        text_file.write(content)


def load_input(year: int, day: int) -> str:
    for input_file in ["input", "problem.md"]:
        if not os.path.exists(f"{CURRENT_DIR}/{input_file}"):
            fetch_and_save(year, day, input_file)

    with open(f"{CURRENT_DIR}/input") as file:
        return file.read()


def load_samples() -> Iterable[Tuple[str, dict[int, int | str]]]:
    any_samples = False
    for sample_file in sorted(os.listdir(CURRENT_DIR)):
        if not sample_file.startswith("sample"):
            continue

        any_samples = True
        path = f"{CURRENT_DIR}/{sample_file}"
        print(f"📄 Found {sample_file}.")

        with open(path) as file:
            sample_date = file.read()
            sample_input, sample_output = sample_date.split("\n---\n")
            sample_outputs = [
                int(v) if v.isnumeric() else v for v in sample_output.split("\n")
            ]
            if sample_file.startswith("sample_1"):
                yield sample_input, {1: sample_outputs[0]}
            elif sample_file.startswith("sample_2"):
                yield sample_input, {2: sample_outputs[0]}
            else:
                yield sample_input, dict(
                    enumerate(filter(lambda x: bool(x), sample_outputs), 1)
                )

    if not any_samples:
        print(f"{Fore.YELLOW}🫣 Could not find any sample files.{Style.RESET_ALL}")
        return True


def is_answer_possible(answer, part_state: dict) -> bool:
    attempts = part_state["attempts"]
    lower_bound = part_state.get("lower_bound")
    upper_bound = part_state.get("upper_bound")

    if answer in attempts:
        print(f"{Fore.RED}❌ Already tried this value.{Style.RESET_ALL}")
        return False

    if lower_bound is not None and answer < lower_bound:
        print(f"{Fore.RED}❌ Answer is too low.{Style.RESET_ALL}")
        return False

    if upper_bound is not None and answer > upper_bound:
        print(f"{Fore.RED}❌ Answer is too high.{Style.RESET_ALL}")
        return False

    return True


def check_answer_and_submit(answer: int, part: int, year: int, day: int) -> None:
    state = load_state()
    part_state = state[str(part)]

    if part_state["solved"]:
        log_correct_or_wrong(answer, part_state["solution"])
        print(f"{Fore.BLUE}⏭️ Already solved, skipping submission.{Style.RESET_ALL}")
        return

    is_possible = is_answer_possible(answer, part_state)
    part_state["attempts"].append(answer)
    save_state(state)

    if not is_possible:
        print(f"{Fore.BLUE}⏭️ Skipping submission.{Style.RESET_ALL}")
        return

    print(f"{Fore.BLUE}📬 Submitting solution now.{Style.RESET_ALL}")
    data = {"level": str(part), "answer": str(answer)}
    response = requests.post(
        f"https://adventofcode.com/{year}/day/{day}/answer", headers=HEADERS, data=data
    )
    soup = BeautifulSoup(response.text, "html.parser")
    message = soup.article.text

    if "that's the right answer" in message.lower():
        print(f"{Fore.GREEN}✅ Correct!{Style.RESET_ALL}")
        save_solution(answer, part_state)

        if part == 1:
            print("Updated problem with part 2:\n\n")
            print(fetch_and_save(year, day, "problem.md"))
    elif "not the right answer" in message.lower():
        print(f"{Fore.RED}❌ Wrong answer! For details:\n{Style.RESET_ALL}")
        print(message)
        if "too low" in message:
            lower_bound = answer + 1
            prev_lower_bound = part_state.get("lower_bound")
            if prev_lower_bound:
                part_state["lower_bound"] = max(lower_bound, prev_lower_bound)
            else:
                part_state["lower_bound"] = lower_bound
        if "too high" in message:
            upper_bound = answer - 1
            prev_upper_bound = part_state.get("upper_bound")
            if prev_upper_bound:
                part_state["upper_bound"] = min(upper_bound, prev_upper_bound)
            else:
                part_state["upper_bound"] = upper_bound
    elif "answer too recently" in message.lower():
        print(f"{Fore.YELLOW}🚫 You gave an answer too recently{Style.RESET_ALL}")
    elif "already complete it" in message.lower():
        print(f"{Fore.YELLOW}⚠️ You have already solved this.{Style.RESET_ALL}")
        save_solution(try_to_parse_solution(year, day, part), part_state)
        log_correct_or_wrong(answer, part_state["solution"])

    save_state(state)


def save_solution(answer: Optional[int | str], part_state: dict) -> None:
    if answer is None:
        return

    part_state["solved"] = True
    part_state["solution"] = answer
    if isinstance(answer, int):
        part_state["lower_bound"] = answer
        part_state["upper_bound"] = answer


def log_correct_or_wrong(answer: [int | str], solution: Optional[int | str]) -> None:
    if solution:
        if solution == answer:
            print(f"{Fore.GREEN}✅ Correct!{Style.RESET_ALL}")
        else:
            print(f"{Fore.RED}❌ Wrong! Should be: {solution}{Style.RESET_ALL}")


def try_to_parse_solution(year: int, day: int, part: int) -> Optional[int | str]:
    url = f"https://adventofcode.com/{year}/day/{day}"
    response = requests.get(url, headers=HEADERS)
    soup = BeautifulSoup(response.text, "html.parser")
    solutions = [
        text.replace(".", "").split(" ")[-1]
        for p in soup.select("p")
        if "Your puzzle answer was" in (text := p.text)
    ]

    index = part - 1
    if index >= len(solutions):
        return None

    solution = solutions[index]
    try:
        return int(solution)
    except ValueError:
        return solution


def save_state(state: dict) -> None:
    with open(f"{os.getcwd()}/state.json", "w+") as state_file:
        state_file.write(json.dumps(state, indent=2))


def sample(answer_func: Callable[[str], Iterable[int | str]]) -> bool:
    print("\nLooking for samples:")

    for sample in load_samples():
        sample_input, sample_output = sample

        for part, actual in enumerate(answer_func(sample_input), 1):
            if part not in sample_output:
                print(
                    f"{Fore.YELLOW}🤷 No sample solution for part {part}.{Style.RESET_ALL}"
                )
                continue

            expected = sample_output[part]
            log_correct_or_wrong(actual, expected)
            if actual != expected:
                return False

    return True


def load_state() -> dict:
    star_path = os.getcwd()
    state_file = f"{star_path}/state.json"
    if not os.path.exists(state_file):
        part_state = {
            "attempts": [],
            "solved": False,
            "solution": None,
            "upper_bound": None,
            "lower_bound": None,
        }
        return {
            "1": deepcopy(part_state),
            "2": deepcopy(part_state),
        }

    with open(state_file) as file:
        return json.loads(file.read())


def handle_error_status(code: int) -> None:
    match code:
        case 404:
            print(f"{Fore.RED}{code}: This day is not available yet!{Style.RESET_ALL}")
            quit()
        case 400:
            print(f"{Fore.RED}{code}: Bad credentials!{Style.RESET_ALL}")
            quit()
        case _ if code > 400:
            print(f"{Fore.RED}{code}: General error!{Style.RESET_ALL}")
            quit()


def solve_for_input(
        answer_func: Callable[[str], Iterable[int | str]],
        parts: tuple[int],
        submit_answer: bool,
) -> None:
    day, year = get_day_and_year()
    problem_input = load_input(year, day)

    print("\nComputing answers for input now:")
    for part, answer in zip(parts, answer_func(problem_input)):
        if not submit_answer:
            print(f"{Fore.BLUE}⏭️ Skipping submission.{Style.RESET_ALL}")

        check_answer_and_submit(answer, part, year, day)


def get_day_and_year() -> tuple[int, int]:
    year, day = [int(v) for v in CURRENT_DIR.split("/")[-2:]]
    return day, year


def answer_func_with_timings(
        answer_func: Callable[[str], Iterable[int | str]]
) -> Callable[[str], Iterable[int | str]]:
    day, year = get_day_and_year()

    def run_answer_func_with_timings(input_data: str) -> Iterable[int | str]:
        start_time = time()
        for part, answer in enumerate(answer_func(input_data), 1):
            end_time = time()
            execution_time = end_time - start_time
            time_str = (
                f"{execution_time:.2f} s"
                if execution_time > 1
                else f"{execution_time * 1000:.2f} ms"
            )
            print(
                f"🧮 Computed answer {answer} for part {part} of day {day} in {time_str}"
            )
            yield answer
            start_time = time()

    return run_answer_func_with_timings


def run(
        answer_func: Callable[[str], Iterable[int | str]],
        skip_sample: bool = False,
        submit_answer: bool = True,
        parts: tuple[int] = (1, 2),
):
    day, year = get_day_and_year()
    print(f"{Fore.MAGENTA}Advent of Code {year}, Day {day}:{Style.RESET_ALL}")

    answer_func = answer_func_with_timings(answer_func)

    load_input(year, day)

    if not skip_sample and not sample(answer_func):
        print(f"{Fore.RED}🧐 Got wrong answer for sample. Stopping.{Style.RESET_ALL}")
        return

    solve_for_input(answer_func, parts, submit_answer)
