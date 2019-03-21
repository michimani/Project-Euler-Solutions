#
# Solution for Project Euler problem 10
# Copyright michimani All rights reserved.
#
# https://projecteuler.net/problem=10
#
import math
UNTIL = 2000000


def compute():
    ans = 2
    num = 3
    while num <= UNTIL:
        is_prime = True
        for dividor in range(3, math.floor(num ** (1/2)) + 1, 2):
            if num % dividor == 0:
                is_prime = False
                break
        if is_prime is True:
            ans = ans + num

        print('{} %'.format(round((num / UNTIL), 2) * 100))
        num = num + 2
    return ans


if __name__ == "__main__":
    print(compute())
