#!/usr/bin/env python3

import json
import os
import re
import shlex
import shutil
import subprocess
import sys
from io import TextIOWrapper
from pathlib import Path
from threading import Thread
from typing import AnyStr, Dict, Generator, List, Literal, Optional, Tuple, Union

import requests

script = Path(__file__)

root_dir = script.parent.parent.parent
programs_dir = root_dir / "programs"
output_dir = root_dir / ".bin"


LEVELS = ["ERROR", "WARN", "INFO", "DEBUG", "TRACE"]
PROGRAM_LOG = "Program log:"
PROGRAM_SOURCES = [PROGRAM_LOG, "Program", "process_instruction:", "solana_runtime:"]

LOG_RE = re.compile(f"^.+ ({'|'.join(LEVELS)})( *) (.+)] ?(?:({'|'.join(PROGRAM_SOURCES)}) )?(.+)$")

ENABLE_COLOR = os.getenv("NO_COLOR") is None and os.getenv("TERM") != "dumb" and sys.stdout.isatty()
STYLE_RESET = "\x1b[0m"


def get_latest_platform_tools() -> Optional[str]:
    try:
        res = requests.get("http://api.github.com/repos/anza-xyz/platform-tools/tags")
        data = json.loads(res.text)
    except requests.RequestException | json.JSONDecodeError:
        return None

    try:
        return data[0]["name"]
    except KeyError:
        return None


def get_tools_version_args() -> Union[Tuple[Literal["--tools-version"], str], Tuple[()]]:
    version = get_latest_platform_tools()

    if version is None:
        return ()
    else:
        return ("--tools-version", version)


def get_program_dirs() -> Generator[Path, None, None]:
    for file in programs_dir.iterdir():
        manifest = file / "Cargo.toml"

        if manifest.is_file():
            yield file


def get_inactive_features() -> List[str]:
    solana = shutil.which("solana")
    if solana is None:
        raise RuntimeError("solana executable not found")

    feature_statuses = subprocess.check_output(
        [
            solana,
            "feature",
            "status",
            "--display-all",
            "--url",
            "mainnet-beta",
            "--output",
            "json",
        ],
    )

    status = json.loads(feature_statuses)
    features: List[Dict[str, str]] = status["features"]

    inactive = filter(lambda feature: feature["status"] == "inactive", features)
    return [feature["id"] for feature in inactive]


def split_args(args: List[str]) -> Tuple[List[str], List[str]]:
    if "--" in args:
        split = args.index("--")
        return args[:split], args[split + 1 :]
    return args, []


def colorize(color: str, text: str) -> str:
    return f"\x1b[{color}m{text}{STYLE_RESET}"


def log_color(log_level: str, log_source: str, program_source: str | None, program_log: str) -> str:
    log = program_log.lower()
    if (
        "error: " in log
        or "error " in log
        or "err: " in log
        or "err " in log
        or "failure: " in log
        or "failure " in log
        or "failed: " in log
        or "failed " in log
        or "fail: " in log
        or "fail " in log
    ):
        return "31"  # red

    match log_level:
        case "ERROR":
            return "31"  # red
        case "WARN":
            return "33"  # yellow
        case "INFO":
            return "32"  # green
        case "DEBUG" if program_source == PROGRAM_LOG:
            return "37"  # white
        case "DEBUG" if "signer privilege escalated" in program_log:
            return "1;30"  # bold light grey
        case "DEBUG" if log_source.endswith("stable_log"):
            return "30"  # light grey
        case "DEBUG":
            return "90"  # grey
        case _:
            return "2;30"  # dim grey


def color_level(log_level: str) -> str:
    match log_level:
        case "ERROR":
            return colorize("1;31", log_level)  # red
        case "WARN":
            return colorize("1;33", log_level)  # yellow
        case "INFO":
            return colorize("1;32", log_level)  # green
        case "DEBUG":
            return colorize("1;34", log_level)  # blue
        case "TRACE":
            return colorize("1;2;30", log_level)  # dim grey
        case _:
            return log_level


def format_log(line: str) -> str:
    matches = LOG_RE.match(line)
    if matches is None:
        return line

    log_level = matches[1]
    log_level_align = matches[2]
    log_source = matches[3]
    program_source = matches[4]
    program_log = matches[5]

    if program_source is None or program_source == PROGRAM_LOG:
        msg = program_log
    else:
        msg = f"{program_source} {program_log}"

    if ENABLE_COLOR:
        msg = colorize(log_color(log_level, log_source, program_source, program_log), msg)
        log_level = color_level(log_level)

    return f"  {log_level_align}{log_level}  {msg}\n"


def test(
    program: Path, cmd: List[AnyStr | Path], env: Dict[str, AnyStr], cwd: Optional[Path] = None
):
    with subprocess.Popen(
        cmd,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        close_fds=True,
        encoding="utf-8",
        cwd=(cwd or program),
        env=env,
    ) as p:
        stdout = p.stdout
        stderr = p.stderr

        if not isinstance(stdout, TextIOWrapper):
            raise RuntimeError("stdout is not a TextIOWrapper")
        if not isinstance(stderr, TextIOWrapper):
            raise RuntimeError("stderr is not a TextIOWrapper")

        def pipe_stdout(f: TextIOWrapper):
            for line in f:
                sys.stdout.write(line)
                sys.stdout.flush()

        def pipe_stderr(f: TextIOWrapper):
            for line in f:
                sys.stderr.write(format_log(line))
                sys.stderr.flush()

        t1 = Thread(target=pipe_stdout, args=[stdout])
        t2 = Thread(target=pipe_stderr, args=[stderr])

        t1.start()
        t2.start()

        t1.join()
        t2.join()

    if p.returncode != 0:
        print(f"\ntests for {program.name} exited with a non-zero exit status")
        exit(1)


def main(args: List[str]):
    cargo = shutil.which("cargo")
    if cargo is None:
        raise RuntimeError("cargo executable not found")

    program_filter, args = split_args(args)
    cargo_args, test_args = split_args(args)

    if len(program_filter) == 0:
        programs_env = os.getenv("PROGRAMS")
        if programs_env is not None:
            program_filter = shlex.split(programs_env)

    programs = get_program_dirs()
    if len(program_filter) > 0:
        programs = filter(lambda f: f.name in program_filter, programs)
    programs = list(programs)

    if len(programs) == 0:
        print("no programs found")
        exit(1)

    env = os.environ.copy()

    color = "always" if ENABLE_COLOR else "never"
    env["CARGO_TERM_COLOR"] = color

    inactive_features = get_inactive_features()
    env["MAINNET_INACTIVE_FEATURES"] = "\n".join(inactive_features)

    # if "RUST_BACKTRACE" not in env or env["RUST_BACKTRACE"] == "":
    #     env["RUST_BACKTRACE"] = "1"

    tools_version = get_tools_version_args()

    cmd = [
        cargo,
        "test-sbf",
        "--sbf-out-dir",
        output_dir,
        *tools_version,
        *cargo_args,
        "--",
        "--test-threads",
        "1",
        "--color",
        color,
        *test_args,
    ]

    for program in programs:
        test(program, cmd, env)


if __name__ == "__main__":
    main(sys.argv[1:])
