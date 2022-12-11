# aoc_day7a.py

with open('aoc_day7.txt') as file:
    line = file.readline()

crabs = [int(i) for i in line.split(',')]

highest = max(crabs)
lowest = min(crabs)

total = highest * len(crabs)
for i in range(lowest, highest + 1):
    new_total = 0
    for crab in crabs:
        new_total += abs(crab - i)
    total = min(total, new_total)

print(total)
