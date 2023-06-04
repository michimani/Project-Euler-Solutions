#
# Solution for Project Euler problem 15
# Copyright michimani All rights reserved.
#
# https://projecteuler.net/problem=15
#

import math

COL = 20
ROW = 20


def compute():
    return round(
        math.factorial(COL + ROW) / ((math.factorial(COL) * math.factorial(ROW)))
    )


if __name__ == "__main__":
    print(compute())
