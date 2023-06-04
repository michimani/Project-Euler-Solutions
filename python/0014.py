#
# Solution for Project Euler problem 14
# Copyright michimani All rights reserved.
#
# https://projecteuler.net/problem=14
#

MAX_NUMBER = 1000000


def compute() -> int:
    checked: dict = {}
    ans_start_num = 0
    max_seq_length = 0
    for start_num in range(1, MAX_NUMBER):
        seq_length = get_collatz_sequence_length(start_num, checked)
        checked[start_num] = seq_length

        if seq_length > max_seq_length:
            max_seq_length = seq_length
            ans_start_num = start_num

        start_num = start_num - 1

    return ans_start_num


def get_collatz_sequence_length(start_number, checked):
    next_num = start_number
    num_sequence = [next_num]
    contains_checked = False
    while next_num != 1:
        if next_num in checked:
            contains_checked = True
            break

        if next_num % 2 == 0:
            next_num = round(next_num / 2)
        else:
            next_num = next_num * 3 + 1

        num_sequence.append(next_num)

    seq_length = len(num_sequence)
    if contains_checked is True:
        seq_length = seq_length + checked[next_num] - 1

    return seq_length


if __name__ == "__main__":
    print(compute())
