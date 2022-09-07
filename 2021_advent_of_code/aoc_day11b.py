# aoc_day11b.py

def valid(data, i, j):
    return 0<=i<len(data) and 0<=j<len(data[0])

def flash(data, flashed, i, j):
    flashed[i][j] = True
    data[i][j] = 0

    for dx, dy in dirs:
        if valid(data, i+dx, j+dy) and not flashed[i+dx][j+dy]:
            data[i+dx][j+dy] += 1
            if data[i+dx][j+dy] > 9:
                flash(data, flashed, i+dx, j+dy)

dirs = ( (1, 1), (1, 0), (1, -1), (0, 1), (0, -1), (-1, 1), (-1, 0), (-1, -1) )

with open('aoc_day11.txt') as file:
    data = [[int(i) for i in l] for l in file.read().split('\n')]

count = 0
trip = False
while not trip:
    data = [[e+1 for e in l] for l in data]

    flashed = [[False for e in l] for l in data]

    for i, line in enumerate(data):
        for j, char in enumerate(line):
            if char > 9:
                flash(data, flashed, i, j)

    trip = True
    for row in flashed:
        for col in row:
            if not col:
                trip = False
    
    count += 1

print(count)
