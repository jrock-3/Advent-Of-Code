# aoc_day7_optimal.py

with open('aoc_day7.txt') as file:
    crabs = [int(i) for i in file.readline().split(',')]

crabs.sort()

l = min(crabs)

total = sum([abs(crab - l) * (abs(crab - l) + 1) // 2 for crab in crabs])

### REAL CODE HERE

idx = 0
for i in range(l, max(crabs) + 1):
    
    new_total = total

    new_total += ???
    new_total -= ???
    
    total = min(total, new_total)

print(total)
