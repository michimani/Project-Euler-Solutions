#
# Solution for Project Euler problem 26
# Copyright michimani All rights reserved.
#
# https://projecteuler.net/problem=26
#

LIMIT_NUM = 1000


def compute():
    ans = 1
    tmp_length = 0
    for nature_num in range(1, 1000):
        repeat_part = get_repeating_part(nature_num)
        if len(repeat_part) > tmp_length:
            ans = nature_num
            tmp_length = len(repeat_part)

    return ans


def get_repeating_part(nature_number):
    disits = list()
    divided = 1
    div = nature_number
    quot = 0
    remain = 0
    remain_list = [1]

    while True:
        if divided // div == 0:
            divided = divided * 10

        remain = divided % div
        quot = divided // div

        disits.append(str(quot))
        divided = remain

        if remain == 0 or remain in remain_list:
            break

        remain_list.append(remain)

    if remain == 0:
        return ""
    else:
        first_appear_idx = remain_list.index(remain)
        return "".join(disits[first_appear_idx:])


if __name__ == "__main__":
    print(compute())
