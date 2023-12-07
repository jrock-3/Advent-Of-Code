# aoc_day3a.py

num_bits = list()

with open('aoc_day3.txt') as file:
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
