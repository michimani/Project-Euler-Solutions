#
# Solution for Project Euler problem 2
# Copyright michimani All rights reserved.
#
# https://projecteuler.net/problem=2
#


def solve():
    seq = [1, 2]
    last_num = 0
    seq_idx = 2
    ans = 2
    while last_num <= 4000000:
        pre = seq[seq_idx - 1]
        p_pre = seq[seq_idx - 2]
        last_num = pre + p_pre
        seq.append(last_num)
        if last_num % 2 == 0 and last_num <= 4000000:
            ans = ans + last_num

        seq_idx = seq_idx + 1

    return ans


if __name__ == "__main__":
    print(solve())
