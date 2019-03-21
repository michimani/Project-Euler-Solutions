#
# Create solution script
#

import os
import sys

SOLUTION_TEMPLATE = """#
# Solution for Project Euler problem PROBLEM_NUM
# Copyright michimani All rights reserved.
#
# https://projecteuler.net/problem=PROBLEM_NUM
#
"""


def create_new_solution(problem_no):
    file_path = '{}/{:04}.py'.format(
        os.path.dirname(os.path.abspath(__file__)), problem_no)
    if os.path.exists(file_path):
        print('{} is already exists.'.format(file_path))
        return True

    solution_script_base = SOLUTION_TEMPLATE.replace(
        'PROBLEM_NUM', str(problem_no))

    with open(file_path, 'a') as f:
        f.write(solution_script_base)


if __name__ == "__main__":
    arg = sys.argv
    if len(arg) < 2:
        print('The first parameter is required.')
    else:
        create_new_solution(int(arg[1]))
