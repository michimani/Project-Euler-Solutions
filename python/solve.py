import sys
from typing import Final
from util.measure import timeit
from solution import (
    s0001,
    s0002,
    s0003,
    s0004,
    s0005,
    s0006,
    s0007,
    s0008,
    s0009,
    s0010,
    s0011,
    s0012,
    s0013,
    s0014,
    s0015,
    s0016,
    s0017,
    s0018,
    s0019,
    s0020,
    s0021,
    s0026,
)


SOLVED_SOLUTIONS: Final[dict] = {
    "1": s0001.solve,
    "2": s0002.solve,
    "3": s0003.solve,
    "4": s0004.solve,
    "5": s0005.solve,
    "6": s0006.solve,
    "7": s0007.solve,
    "8": s0008.solve,
    "9": s0009.solve,
    "10": s0010.solve,
    "11": s0011.solve,
    "12": s0012.solve,
    "13": s0013.solve,
    "14": s0014.solve,
    "15": s0015.solve,
    "16": s0016.solve,
    "17": s0017.solve,
    "18": s0018.solve,
    "19": s0019.solve,
    "20": s0020.solve,
    "21": s0021.solve,
    "26": s0026.solve,
}


@timeit
def solve(problem_no: str):
    print(SOLVED_SOLUTIONS[problem_no]())


if __name__ == "__main__":
    args = sys.argv

    if len(args) < 2:
        print("Please input problem number.")
        sys.exit(1)

    problem_no = args[1]

    if problem_no not in SOLVED_SOLUTIONS:
        print("solution for problem '{}' does not found.".format(problem_no))
        sys.exit(1)

    solve(args[1])
