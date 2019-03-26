#
# Solution for Project Euler problem 17
# Copyright michimani All rights reserved.
#
# https://projecteuler.net/problem=17
#

FOR_NUMBER = 1000

NUM_TO_WORD = {
    1: {'1st': 'one'},
    2: {'1st': 'two', '2nd': 'twen'},
    3: {'1st': 'three', '2nd': 'thir'},
    4: {'1st': 'four', '2nd': 'for'},
    5: {'1st': 'five', '2nd': 'fif'},
    6: {'1st': 'six', '2nd': 'six'},
    7: {'1st': 'seven', '2nd': 'seven'},
    8: {'1st': 'eight', '2nd': 'eigh'},
    9: {'1st': 'nine', '2nd': 'nine'},
    10: {'1st': 'ten', '2nd': 'hundred'}
}


def compute():
    string = ''
    for num in range(1, FOR_NUMBER + 1):
        num_string = convert_number_to_string(num)
        # print(num, num_string)
        string = string + num_string.replace('-', '').replace(' ', '')
    return len(string)


def convert_number_to_string(number):
    num_str = str(number)
    number_string = ''
    digit_1 = 0
    digit_10 = 0
    digit_100 = 0
    digit_1000 = 0
    if number == 0:
        number_string = ''

    elif number == 11:
        number_string = 'eleven'

    elif number == 12:
        number_string = 'twelve'

    elif number == 14:
        number_string = 'fourteen'

    elif len(num_str) == 1 or number == 10:
        number_string = NUM_TO_WORD[number]['1st']

    elif len(num_str) == 2:
        digit_1 = int(list(num_str)[1])
        digit_10 = int(list(num_str)[0])
        if digit_10 == 1:
            number_string = NUM_TO_WORD[digit_1]['2nd'] + 'teen'

        else:
            number_string = NUM_TO_WORD[digit_10]['2nd'] + 'ty'
            if digit_1 > 0:
                number_string = number_string + \
                    '-' + NUM_TO_WORD[digit_1]['1st']

    elif len(num_str) == 3:
        digit_100 = int(list(num_str)[0])
        number_string = NUM_TO_WORD[digit_100]['1st'] + ' hundred'
        digit_1_to_10 = convert_number_to_string(int(num_str[1:]))
        if digit_1_to_10 != '':
            number_string = number_string + ' and ' + digit_1_to_10

    elif len(num_str) == 4:
        digit_1000 = int(list(num_str)[0])
        number_string = NUM_TO_WORD[digit_1000]['1st'] + ' thousand'
        digit_1_to_100 = convert_number_to_string(int(num_str[1:]))
        if digit_1_to_100 != '':
            number_string = number_string + ' and ' + digit_1_to_100

    return number_string


if __name__ == "__main__":
    print(compute())
