import os
from typing import Callable, Tuple, Iterable

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
            if sample_file == "sample":
                yield sample_input, dict(enumerate(sample_outputs))
            if sample_file.startswith("sample_1"):
                yield sample_input, {1: sample_outputs[0]}
            if sample_file.startswith("sample_2"):
                yield sample_input, {2: sample_outputs[0]}

    if not any_samples:
        print(f"{Fore.YELLOW}🫣 Could not find any sample files.{Style.RESET_ALL}")
        return True


def submit(answer: int, level: int, year: int, day: int) -> None:
    print(f"{Fore.BLUE}📬 Submitting solution now.{Style.RESET_ALL}")
    data = {"level": str(level), "answer": str(answer)}
    response = requests.post(
        f"https://adventofcode.com/{year}/day/{day}/answer", headers=HEADERS, data=data
    )
    soup = BeautifulSoup(response.text, "html.parser")
    message = soup.article.text

    if "that's the right answer" in message.lower():
        print(f"{Fore.GREEN}✅ Correct!{Style.RESET_ALL}")
        save_stars(level)

        if level == 1:
            print("Updated problem with part 2:\n\n")
            print(fetch_and_save(year, day, "problem.md"))
    elif "not the right answer" in message.lower():
        print(f"{Fore.RED}❌ Wrong answer! For details:\n{Style.RESET_ALL}")
        print(message)
    elif "answer too recently" in message.lower():
        print(f"{Fore.YELLOW}🚫 You gave an answer too recently{Style.RESET_ALL}")
    elif "already complete it" in message.lower():
        print(f"{Fore.YELLOW}⚠️ You have already solved this.{Style.RESET_ALL}")
        save_stars(level)


def save_stars(level: int) -> None:
    star_path = os.getcwd()
    with open(f"{star_path}/stars", "w+") as star_file:
        stars = "*" * level
        print(f"Writing '{stars}' to star file...")
        star_file.write(stars)


def test(answer_func: Callable[[str], Iterable[int | str]], cases: list[dict]) -> bool:
    all_passed = True

    if not cases:
        print(f"{Fore.YELLOW}🤷 No test cases defined.{Style.RESET_ALL}")
        return all_passed

    for tc in cases:
        answer = answer_func(tc["input"])
        if str(tc["output"]) == str(answer):
            print(
                f"{Fore.GREEN}🎄 Test passed {Style.RESET_ALL}[Part {tc['level']}] Input: '{tc['input']}'; Output: '{tc['output']}'"
            )
        else:
            all_passed = False
            print(
                f"{Fore.RED}🔥 Test failed {Style.RESET_ALL}[Part {tc['level']}] Input: '{tc['input']}'; Submitted: '{answer}'; Correct: '{tc['output']}'"
            )

    return all_passed


def sample(answer_func: Callable[[str], Iterable[int | str]]) -> bool:
    print("\n👀 Looking for samples:")

    for sample in load_samples():
        sample_input, sample_output = sample

        for part, actual in enumerate(answer_func(sample_input), 1):
            if part not in sample_output:
                continue

            expected = sample_output[part]
            print(
                f"{Fore.BLUE}🧮 Computed sample answer {actual} "
                f"(expected {expected}) for part {part}.{Style.RESET_ALL}"
            )
            if actual != expected:
                return False

    return True


def check_stars() -> int:
    star_path = os.getcwd()
    star_file = f"{star_path}/stars"
    if not os.path.exists(star_file):
        return 0

    with open(star_file, "r") as file:
        stars = file.read().strip()
        return len(stars)


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


def run(
        answer_func: Callable[[str], Iterable[int | str]],
        test_cases=None,
        skip_sample: bool = False,
        submit_answer: bool = True,
        parts: tuple[int] = (1, 2),
):
    year, day = [int(v) for v in CURRENT_DIR.split("/")[-2:]]
    print(f"{Fore.MAGENTA}Advent of Code {year}, Day {day}:{Style.RESET_ALL}")
    problem_input = load_input(year, day)

    if not skip_sample and not sample(answer_func):
        print(f"{Fore.RED}🧐 Got wrong answer for sample. Stopping.{Style.RESET_ALL}")
        return

    if not test(answer_func, test_cases):
        print(f"{Fore.RED}🧪 Tests failed. Stopping.{Style.RESET_ALL}")
        return

    stars = check_stars()

    print("\nComputing answers for input now:")
    for part, answer in zip(parts, answer_func(problem_input)):
        print(f"🧮 Computed answer {answer} for part {part} of day {day}")
        if not submit_answer:
            print(f"{Fore.BLUE}⏭️ Skipping submission.{Style.RESET_ALL}")
        elif stars < part:
            submit(answer, part, year, day)
        else:
            print(
                f"{Fore.BLUE}⏭️ Already solved, skipping submission.{Style.RESET_ALL}"
            )
