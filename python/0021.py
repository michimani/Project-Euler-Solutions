#
# Solution for Project Euler problem 21
# Copyright michimani All rights reserved.
#
# https://projecteuler.net/problem=21
#


def compute():
    ans = 0
    amicable_number_list = list()

    for num in range(1, 10000):
        if num in amicable_number_list:
            continue

        d_a = d(num)
        d_a_b = d(d_a)
        if d_a_b == num and num != d_a:
            amicable_number_list.append(num)
            if d_a not in amicable_number_list:
                amicable_number_list.append(d_a)

    if len(amicable_number_list) > 0:
        ans = sum(amicable_number_list)

    return ans


def d(num):
    sum_of_pd = 0  # sum of proper divisors
    for n in range(1, num):
        if num % n == 0:
            sum_of_pd += n

    return sum_of_pd


if __name__ == "__main__":
    print(compute())
