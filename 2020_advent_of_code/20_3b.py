# 20_3b.py

import math

counts = [0, 0, 0, 0, 0]

with open('20_3.txt') as file:
    file.readline()

    line = file.readline().strip('\n')

    x = 1
    while line != '':
        for i in range(len(counts) - 1):
            if line[((1 + 2 * i) * x) % len(line)] == '#':
                counts[i] += 1

        if x % 2 == 0 and line[x // 2 % len(line)] == '#':
            counts[-1] += 1
        
        x += 1
        line = file.readline().strip('\n')

print(math.prod(counts))
