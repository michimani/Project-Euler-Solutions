#
# Solution for Project Euler problem 4
# Copyright michimani All rights reserved.
#
# https://projecteuler.net/problem=4
#

DIGIT = 3


def solve():
    palindrome_product_list = list()
    for a in range(10 ** (DIGIT - 1), 10**DIGIT):
        for b in range(10 ** (DIGIT - 1), 10**DIGIT):
            product = a * b
            if is_palindrome(product) is True:
                palindrome_product_list.append(product)

    return max(palindrome_product_list)


def is_palindrome(number):
    return str(number) == "".join(reversed(list(str(number))))


if __name__ == "__main__":
    print(solve())
