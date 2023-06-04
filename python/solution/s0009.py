#
# Solution for Project Euler problem 9
# Copyright michimani All rights reserved.
#
# https://projecteuler.net/problem=9
#

import math


def solve():
    is_ok = False
    a = 1
    b = 2
    c = 3
    max_c = 1000 - a - b
    sq_list = [num**2 for num in range(3, max_c + 1)]
    for c_sq in sq_list:
        for a_sq in range(1, math.ceil(c_sq / 2)):
            if is_nature_sqrt(a_sq) is True:
                b_sq = c_sq - a_sq
                if is_nature_sqrt(b_sq) is True:
                    a = round(a_sq ** (1 / 2))
                    b = round(b_sq ** (1 / 2))
                    c = round(c_sq ** (1 / 2))
                    if a + b + c == 1000:
                        is_ok = True
                        break

        if is_ok is True:
            break

    return a * b * c


def is_nature_sqrt(num):
    if num > 0:
        return num == math.floor(num ** (1 / 2)) ** 2
    else:
        return False


if __name__ == "__main__":
    print(solve())
