#
# Create solution script
#

import os
import sys
from typing import Final

SOLUTION_TEMPLATE: Final[
    str
] = """#
# Solution for Project Euler problem {problem_no}
# Copyright michimani All rights reserved.
#
# https://projecteuler.net/problem={problem_no}
#


def solve() -> int:
    ans = 0
    return ans


if __name__ == "__main__":
    print(solve())
"""


def create_new_solution(problem_no):
    file_path = "{}/solution/s{:04}.py".format(
        os.path.dirname(os.path.abspath(__file__)), problem_no
    )
    if os.path.exists(file_path):
        print("{} is already exists.".format(file_path))
        return True

    solution_script_base = SOLUTION_TEMPLATE.format(problem_no=problem_no)

    with open(file_path, "a") as f:
        f.write(solution_script_base)


if __name__ == "__main__":
    arg = sys.argv
    if len(arg) < 2:
        print("The first parameter is required.")
    else:
        create_new_solution(int(arg[1]))
