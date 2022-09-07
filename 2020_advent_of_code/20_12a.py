# 20_12a.py

import math

bearing = {'N': [1, 0], 'S': [-1, 0], 'E': [0, 1], 'W': [0, -1]}

pos = [0, 0]
facing = 0

with open('20_12.txt') as file:
    data = file.read().split('\n')

for line in data:
    i = line[0]
    val = int(line[1:])
    
    if i in bearing:
        dx, dy = bearing[line[0]]
    elif i == 'F':
        rad = math.radians(facing)
        dx, dy = int(math.sin(rad)), int(math.cos(rad))
    elif i == 'R':
        facing -= val
        continue
    elif i == 'L':
        facing += val
        continue

    pos[0] += dx * val
    pos[1] += dy * val

print(abs(pos[0]) + abs(pos[1]))
