#
# Solution for Project Euler problem 16
# Copyright michimani All rights reserved.
#
# https://projecteuler.net/problem=16
#

EXPONENT = 1000


def solve():
    ans = 0
    for digit in list(str(2**EXPONENT)):
        ans = ans + int(digit)

    return ans


if __name__ == "__main__":
    print(solve())
