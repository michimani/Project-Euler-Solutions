#
# Solution for Project Euler problem 12
# Copyright michimani All rights reserved.
#
# https://projecteuler.net/problem=12
#

DIVISOR_LIMIT = 500


def compute():
    triangle_list = list()
    current_divisor_cnt = 0
    triangle = 0
    while current_divisor_cnt < DIVISOR_LIMIT:
        triangle = triangle + len(triangle_list) + 1
        triangle_list.append(triangle)
        divisor_list = get_divisor_list(triangle)
        current_divisor_cnt = len(divisor_list)

    return triangle


def get_divisor_list(number):
    divisor_tmp = 1
    is_end = False
    divisor_list = list()
    while is_end is False and divisor_tmp <= number:
        if number % divisor_tmp == 0:
            quotient = round(number / divisor_tmp)
            if divisor_tmp in divisor_list and quotient in divisor_list:
                is_end = True
            else:
                if divisor_tmp not in divisor_list:
                    divisor_list.append(divisor_tmp)
                if quotient not in divisor_list:
                    divisor_list.append(quotient)

        divisor_tmp = divisor_tmp + 1

    return divisor_list


if __name__ == "__main__":
    print(compute())
