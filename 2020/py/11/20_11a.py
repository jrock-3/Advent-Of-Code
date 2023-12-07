# 20_11a.py

def is_valid(data, i, j, dx, dy):
    return 0<=i+dx<len(data) and 0<=j+dy<len(data[0])

import copy

dirs = ((1, 1), (1, 0), (1, -1), (0, 1), (0, -1), (-1, 1), (-1, 0), (-1, -1))

with open('20_11.txt') as file:
    data = [list(i) for i in file.read().split('\n')]

copied = 'hi'

while copied != data:
##    for line in data:
##        for char in line:
##            print(char,end='')
##        print()
##    print()
    copied = copy.deepcopy(data)
    for i, line in enumerate(copied):
        for j, char in enumerate(line):
            if char == '.':
                continue
            occupied = 0
            for dx, dy in dirs:
                if is_valid(data, i, j, dx, dy) and copied[i+dx][j+dy] == '#':
                    occupied += 1
            if char == 'L' and occupied == 0:
                data[i][j] = '#'
            elif char == '#' and occupied >= 4:
                data[i][j] = 'L'

total = 0
for line in data:
    for char in line:
        if char == '#':
            total += 1
print(total)
