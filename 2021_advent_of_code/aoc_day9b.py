# aoc_day9b.py

direction = ((0, 1), (0, -1), (1, 0), (-1, 0))

def search(heights, x, y):
    total = 1
    for dx, dy in direction:
        if not (0 <= x + dx < len(heights) and 0 <= y + dy < len(heights[0])):
            continue
        test = heights[x + dx][y + dy]
        if not visited[x + dx][y + dy] and test != 9 and heights[x][y] < test:
            visited[x + dx][y + dy] = True
            total += search(heights, x + dx, y + dy)
    return total

starts = list()

with open('aoc_day9.txt') as file:
    heights = [[int(col) for col in list(line)] for line in file.read().split('\n')]

visited = [[False for col in row] for row in heights]

for i, row in enumerate(heights):
    for j, col in enumerate(row):
        for dx, dy in direction:
            if 0 <= i + dx < len(heights) and 0 <= j + dy < len(row) and col >= heights[i + dx][j + dy]:
                break
        else:
            starts.append((i, j))

basins = list()
for x, y in starts:
    basins.append(search(heights, x, y))

basins.sort()

total = 1
for basin in basins[-3:]:
    total *= basin

print(total)
