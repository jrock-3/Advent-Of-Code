# aoc_day9a.py

direction = ((0, 1), (0, -1), (1, 0), (-1, 0))

with open('aoc_day9.txt') as file:
    heights = [[int(col) + 1 for col in list(line)] for line in file.read().split('\n')]

total = 0
for i, row in enumerate(heights):
    for j, col in enumerate(row):
        for dx, dy in direction:
            if 0 <= i + dx < len(heights) and 0 <= j + dy < len(row) and col >= heights[i + dx][j + dy]:
                break
        else:
            total += col

print(total)
