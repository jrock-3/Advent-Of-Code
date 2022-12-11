# 20_10b.py

import math

def valid_combos(l):
    total = 0
    for i in range(min(2, l) + 1):
        total += math.factorial(l) // (math.factorial(l - i) * math.factorial(i))
##    print(l, total)
    return total

with open('20_10.txt') as file:
    adapters = [int(i) for i in file.read().split('\n')]

adapters.extend([0, max(adapters) + 3])

adapters.sort()

##print(adapters)

i = 0
idx = 0
lists = [[]]
while i < len(adapters) - 1:
    diff = adapters[i+1] - adapters[i]
    if diff == 3:
        i += 1
        if len(lists[idx]) != 0:
            lists.append(list())
            idx += 1
    elif diff == 1 and i > 0 and adapters[i] - adapters[i-1] != 3:
        lists[idx].append(adapters[i])

    i += 1

##print(lists)

total = 1
for nums in lists:
##    print(total)
    l = len(nums)
    if l < 1:
        continue
    total *= valid_combos(l)

print(total)
