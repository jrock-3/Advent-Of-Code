# aoc_day3b.py

import copy

def do_work(ratings, is_largest):
    i = 0
    while len(ratings) > 1 and i < len(ratings[0]):
        count = [0, 0]
        for rate in ratings:
            if rate[i] == '1':
                count[1] += 1
            else:
                count[0] += 1

        if count[0] <= count[1]:
            search = '1' if is_largest else '0'
        else:
            search = '0' if is_largest else '1'

        j = 0
        while j < len(ratings):
            if ratings[j][i] != search:
                del ratings[j]
            else:
                j += 1

        i += 1

with open('aoc_day3.txt') as file:
    lines = file.read().split('\n')

o2_rating = copy.deepcopy(lines)
co2_rating = copy.deepcopy(lines)

do_work(o2_rating, True)
do_work(co2_rating, False)

o2_rating = int(o2_rating[0], base=2)
co2_rating = int(co2_rating[0], base=2)

print(o2_rating * co2_rating)

'''
    line = file.readline().strip('\n')

    for _ in range(len(line)):
        num_bits.append([0,0])

    while line != '':
        for i, bit in enumerate(line):
            if bit == '0':
                num_bits[i][0] += 1
            else:
                num_bits[i][1] += 1

        line = file.readline().strip('\n')

gamma = epsilon = ''

for a, b in num_bits:
    if a > b:
        gamma += '0'
        epsilon += '1'
    else:
        gamma += '1'
        epsilon += '0'

gamma = int(gamma, base=2)
epsilon = int(epsilon, base=2)

print(gamma * epsilon)
'''
