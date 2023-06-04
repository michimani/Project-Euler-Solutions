#
# Solution for Project Euler problem 7
# Copyright michimani All rights reserved.
#
# https://projecteuler.net/problem=7
#

TARGET_PRIME_IDX = 10001


def compute() -> int:
    prime_list: list[int] = []
    prime_cnt = 0
    num = 2
    ans = 2
    while prime_cnt < TARGET_PRIME_IDX:
        is_prime = True
        for prime in prime_list:
            if num % prime == 0:
                is_prime = False
                break
        if is_prime is True:
            prime_list.append(num)
            prime_cnt = prime_cnt + 1
            ans = num

        num = num + 1

    return ans


if __name__ == "__main__":
    print(compute())
