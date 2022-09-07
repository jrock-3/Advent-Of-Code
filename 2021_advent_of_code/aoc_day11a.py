# aoc_day11a.py

def _print(data):
    for line in data:
        for char in line:
            print(char, end='')
        print()
    print()

def valid(data, i, j):
    return 0<=i<len(data) and 0<=j<len(data[0])

def flash(data, flashed, i, j):
    flashed[i][j] = True
    data[i][j] = 0

    total = 1
    for dx, dy in dirs:
        if valid(data, i+dx, j+dy) and not flashed[i+dx][j+dy]:
            data[i+dx][j+dy] += 1
            if data[i+dx][j+dy] > 9:
                total += flash(data, flashed, i+dx, j+dy)
    return total

dirs = ( (1, 1), (1, 0), (1, -1), (0, 1), (0, -1), (-1, 1), (-1, 0), (-1, -1) )

with open('aoc_day11.txt') as file:
    data = [[int(i) for i in l] for l in file.read().split('\n')]

_print(data)

t = 0
for x in range(100):
    data = [[e+1 for e in l] for l in data]

    flashed = [[False for e in l] for l in data]

    for i, line in enumerate(data):
        for j, char in enumerate(line):
            if char > 9:
                t += flash(data, flashed, i, j)

print(t)
