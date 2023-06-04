#
# Solution for Project Euler problem 6
# Copyright michimani All rights reserved.
#
# https://projecteuler.net/problem=6
#

FROM_NUM = 1
TO_NUM = 100


def solve():
    sum_of_square = 0
    sum_of_range = 0
    for num in range(FROM_NUM, TO_NUM + 1):
        sum_of_range += num
        sum_of_square += num**2

    square_of_sum = sum_of_range**2

    ans = square_of_sum - sum_of_square
    return ans


if __name__ == "__main__":
    print(solve())
