#
# Solution for Project Euler problem 19
# Copyright michimani All rights reserved.
#
# https://projecteuler.net/problem=19
#

DAYS = [1, 2, 3, 4, 5, 6, 7]  # from Monday to Sunday
MONTH_DAY_CNT_LIST = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
MONTH_DAY_CNT_LIST_IN_LEAP = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]


def compute():
    ans = 0

    initial_year = get_all_day_in_a_year(1900, 1)
    first_day = initial_year[-1] + 1
    for year in range(1901, 2001):
        all_day = get_all_day_in_a_year(year, first_day)
        ans += get_sunday_cnt(year, all_day)
        first_day = all_day[-1] + 1

    return ans


def get_sunday_cnt(year, all_day):
    cnt = 0
    month_day_cnt_list = MONTH_DAY_CNT_LIST
    if is_leap(year) is True:
        month_day_cnt_list = MONTH_DAY_CNT_LIST_IN_LEAP

    month_top = 0
    for month_day_cnt in month_day_cnt_list:
        if all_day[month_top] == 7:
            cnt += 1

        month_top += month_day_cnt

    return cnt


def get_all_day_in_a_year(year, first_day) -> list[int]:
    all_day: list[int] = []
    day_cnt = sum(MONTH_DAY_CNT_LIST)
    if is_leap(year) is True:
        day_cnt = sum(MONTH_DAY_CNT_LIST_IN_LEAP)

    day_idx = first_day - 1
    while len(all_day) < day_cnt:
        if (day_idx + 1) not in DAYS:
            day_idx = 0

        all_day.append(DAYS[day_idx])
        day_idx += 1

    return all_day


def is_leap(year):
    if (year % 400 != 0 and year % 100 == 0) or year % 4 != 0:
        return False
    else:
        return True


if __name__ == "__main__":
    print(compute())
