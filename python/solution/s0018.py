#
# Solution for Project Euler problem 18
# Copyright michimani All rights reserved.
#
# https://projecteuler.net/problem=18
#

# fmt: off
TRIANGLE = [
    ["75"],
    ["95", "64"],
    ["17", "47", "82"],
    ["18", "35", "87", "10"],
    ["20", "04", "82", "47", "65"],
    ["19", "01", "23", "75", "03", "34"],
    ["88", "02", "77", "73", "07", "63", "67"],
    ["99", "65", "04", "28", "06", "16", "70", "92"],
    ["41", "41", "26", "56", "83", "40", "80", "70", "33"],
    ["41", "48", "72", "33", "47", "32", "37", "16", "94", "29"],
    ["53", "71", "44", "65", "25", "43", "91", "52", "97", "51", "14"],
    ["70", "11", "33", "28", "77", "73", "17", "78", "39", "68", "17", "57"],
    ["91", "71", "52", "38", "17", "14", "91", "43", "58", "50", "27", "29", "48"],
    ["63", "66", "04", "68", "89", "53", "67", "30", "73", "16", "69", "87", "40", "31"],
    ["04", "62", "98", "27", "23", "09", "70", "98", "73", "93", "38", "53", "60", "04", "23"],
]
# fmt: on


def solve():
    max_row = len(TRIANGLE)
    current_row = 0
    idx_choices = (0, 0)
    ans = 0
    while current_row < max_row:
        # in left choice
        left_num = int(TRIANGLE[current_row][idx_choices[0]])
        if current_row + 1 < max_row:
            left_num = left_num + max(
                int(TRIANGLE[current_row + 1][idx_choices[0]]),
                int(TRIANGLE[current_row + 1][idx_choices[0] + 1]),
            )

        # in right choice
        right_num = int(TRIANGLE[current_row][idx_choices[1]])
        if current_row + 1 < max_row:
            right_num = right_num + max(
                int(TRIANGLE[current_row + 1][idx_choices[1]]),
                int(TRIANGLE[current_row + 1][idx_choices[1] + 1]),
            )

        if left_num >= right_num:
            add_num = int(TRIANGLE[current_row][idx_choices[0]])
            choiced_idx = idx_choices[0]
        else:
            add_num = int(TRIANGLE[current_row][idx_choices[1]])
            choiced_idx = idx_choices[1]
        ans = ans + add_num
        idx_choices = get_next_idx_choices(choiced_idx)
        current_row = current_row + 1

    return ans


def get_next_idx_choices(current_idx):
    return (current_idx, current_idx + 1)


if __name__ == "__main__":
    print(solve())
