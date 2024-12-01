"""
    It is a six-digit number.
    The value is within the range given in your puzzle input.
    Two adjacent digits are the same (like 22 in 122345).
    Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).

Other than the range rule, the following are true:

    111111 meets these criteria (double 11, never decreases).
    223450 does not meet these criteria (decreasing pair of digits 50).
    123789 does not meet these criteria (no double).

    Input: 273025-767253
"""

all_possible = [str(number) for number in range(273025, 767253)]
# all_possible = ["111111", "223456", "123789"]
possible = []

for number in all_possible:
    valid_increasing = [False, False, False, False, False]
    valid_double = [False, False, False, False, False]

    for i in range(5):
        if number[i] <= number[i+1]:
            valid_increasing[i] = True

    if all(valid_increasing):
        for i in range(5):
            if number[i] == number[i+1]:
                if i ==1 or i == 2 or i == 3:
                    if number[i] != number[i+2] and number[i] != number[i-1]:
                        valid_double[i] = True
                elif i == 4:
                    if number[i] != number[i-1]:
                        valid_double[i] = True
                elif i == 0:
                    if number[i] != number[i+2]:
                        valid_double[i] = True

    if any(valid_double):
        possible.append(number)

print("The number of valid passwords is {}.".format(len(possible)))
