#!/usr/bin/env python3

import os
import shlex
import shutil
import subprocess
import sys
from pathlib import Path
from typing import Generator, List, Tuple

script = Path(__file__)

root_dir = script.parent.parent.parent
programs_dir = root_dir / "programs"
output_dir = root_dir / ".bin"


def get_program_dirs() -> Generator[Path, None, None]:
    for file in programs_dir.iterdir():
        manifest = file / "Cargo.toml"

        if manifest.is_file():
            yield file


def parse_args(args: List[str]) -> Tuple[List[str], List[str]]:
    if "--" in args:
        split = args.index("--")
        return args[:split], args[split + 1 :]
    return args, []


def build(program: Path, args: List[str]):
    cargo = shutil.which("cargo")
    if cargo is None:
        raise RuntimeError("cargo executable not found")

    return subprocess.check_call(
        [cargo, "build-sbf", "--sbf-out-dir", output_dir, *args],
        cwd=program,
    )


def main(args: List[str]):
    program_filter, args = parse_args(args)

    if len(program_filter) == 0:
        programs_env = os.getenv("PROGRAMS")
        if programs_env is not None:
            program_filter = shlex.split(programs_env)

    programs = get_program_dirs()
    if len(program_filter) > 0:
        programs = filter(lambda f: f.name in program_filter, programs)
    programs = list(programs)

    if len(programs) == 0:
        raise RuntimeError("no programs found")

    for program in programs:
        build(program, args)


if __name__ == "__main__":
    main(sys.argv[1:])
