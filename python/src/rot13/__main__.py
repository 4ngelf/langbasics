import sys
from pathlib import Path
import argparse

from rot13 import rot13, VERSION


def parse_args():
    prog = Path(sys.argv[0]).name
    parser = argparse.ArgumentParser(
        prog=prog, description="ROT13 encoding utility"
    )
    parser.add_argument(
        "-v",
        "--version",
        action="version",
        version=f"%(prog)s(python) v{VERSION}",
    )
    parser.add_argument("TEXT", nargs="*", default="-", help="text to encode")
    return parser.parse_args()


def encode_stdin():
    for line in sys.stdin:
        print(rot13(line.rstrip("\n")))


def main():
    for arg in parse_args().TEXT:
        if arg == "-":
            encode_stdin()
            continue
        print(rot13(arg))


if __name__ == "__main__":
    main()
