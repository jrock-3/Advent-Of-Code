# aoc_day7b.py

with open('aoc_day7.txt') as file:
    line = file.readline()

crabs = [int(i) for i in line.split(',')]

highest = max(crabs)
lowest = min(crabs)

total = highest * (highest + 1) // 2 * len(crabs)
for i in range(lowest, highest + 1):
    new_total = 0
    for crab in crabs:
        x = abs(crab - i)
        new_total += x * (x + 1) // 2
    total = min(total, new_total)

print(total)
