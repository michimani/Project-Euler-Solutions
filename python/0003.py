#
# Solution for Project Euler problem 3
# Copyright michimani All rights reserved.
#
# https://projecteuler.net/problem=3
#


def compute():
    max_num = 600851475143
    quot = max_num
    primes = []
    ans = 0
    for prime_chk in range(2, max_num):
        is_prime = True
        for prime in primes:
            if prime_chk % prime == 0:
                is_prime = False
                break

        if is_prime is True:
            primes.append(prime_chk)
            while quot % prime_chk == 0:
                quot = int(quot / prime_chk)

            if quot <= 1:
                ans = prime_chk
                break

    return ans


if __name__ == "__main__":
    print(compute())
