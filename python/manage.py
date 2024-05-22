#!/usr/bin/env python3
import sys
import argparse
import functools
from pathlib import Path
from subprocess import run as run_default

run = functools.partial(run_default, stdout=sys.stdout, stderr=sys.stderr)


def parse_args() -> argparse.Namespace:
    prog = Path(sys.argv[0]).name
    parser = argparse.ArgumentParser(
        prog=prog, description=f"{prog} - project manager"
    )
    subparsers = parser.add_subparsers(required=True, dest="operation")

    subparsers.add_parser("run", help="run project")
    subparsers.add_parser("test", help="test project")
    install_parser = subparsers.add_parser(
        "install", help="install this project"
    )

    # TODO: add necessary arguments
    install_parser.add_argument(
        "--root", help="directory to install project into"
    )

    return parser.parse_args()


# TODO: match against parsed args and then invoke processes with run()
def main() -> int:
    args = parse_args()

    print(args)

    return 0


if __name__ == "__main__":
    sys.exit(main())
