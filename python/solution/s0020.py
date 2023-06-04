#
# Solution for Project Euler problem 20
# Copyright michimani All rights reserved.
#
# https://projecteuler.net/problem=20
#


def solve():
    ans = 0
    times = 1
    for n in range(1, 101):
        times = times * n

    for digit in list(str(times)):
        ans = ans + int(digit)

    return ans


if __name__ == "__main__":
    print(solve())
