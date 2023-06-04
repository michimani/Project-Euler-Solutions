#
# Solution for Project Euler problem 1
# Copyright michimani All rights reserved.
#
# https://projecteuler.net/problem=1
#


def compute():
    ans = 0
    for num in range(1, 1000):
        if num % 3 == 0 or num % 5 == 0:
            ans = ans + num

    return ans


if __name__ == "__main__":
    print(compute())
