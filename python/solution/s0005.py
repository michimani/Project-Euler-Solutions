#
# Solution for Project Euler problem 5
# Copyright michimani All rights reserved.
#
# https://projecteuler.net/problem=5
#

FROM_NUM = 1
TO_NUM = 20


def solve():
    ans = 1
    checked_number_list = list()
    product_list = list()
    for num in reversed(range(FROM_NUM, TO_NUM + 1)):
        if num == 1:
            continue

        divisor_list = get_divisor_list(num)
        number_exists = True
        for divisor in divisor_list:
            if len(divisor_list) > 2 and divisor == num:
                continue
            elif divisor not in checked_number_list:
                number_exists = False
                checked_number_list.append(divisor)

        if number_exists is False:
            product_list.append(num)
        elif len(divisor_list) > 2:
            is_creatable = False
            for checked_number in checked_number_list:
                if num * checked_number in product_list:
                    is_creatable = True
                    product_list.remove(num * checked_number)
            if is_creatable is True:
                product_list.append(num)

    for product in product_list:
        ans = ans * product
    return ans


def get_divisor_list(number):
    divisor_list = list()
    for num in range(1, number + 1):
        if number % num == 0:
            divisor_list.append(num)

    return divisor_list


def get_prime_list(from_num, to_num) -> list[int]:
    if from_num == 1:
        from_num = 2

    prime_list: list[int] = []
    for num in range(from_num, to_num + 1):
        is_prime = True
        for prime in prime_list:
            if num % prime == 0:
                is_prime = False
                break

        if is_prime is True:
            prime_list.append(num)

    return prime_list


if __name__ == "__main__":
    print(solve())
