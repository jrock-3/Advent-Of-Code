# aoc_day14a.py

import copy

def step(count_pairs, count_chars, rules):
    _count_pairs = copy.deepcopy(count_pairs)
    keys = [k for k, v in count_pairs.items()]
    for i, key in enumerate(keys):
        value = _count_pairs[key]
        if key in rules:
            count_pairs[key] -= value
            if count_pairs[key] == 0:
                del count_pairs[key]

            x = rules[key]
            if x in count_chars:
                count_chars[x] += value
            else:
                count_chars[x] = value
            
            first = key[0] + x
            if first in count_pairs:
                count_pairs[first] += value
            else:
                count_pairs[first] = value

            second = x + key[1]
            if second in count_pairs:
                count_pairs[second] += value
            else:
                count_pairs[second] = value

with open('aoc_day14.txt') as file:
    data = file.read().split('\n')

template = data.pop(0)
data.pop(0)

rules = dict()

for line in data:
    pair, insert = line.split(' -> ')
    rules[pair] = insert

count_pairs = dict()

for i in range(len(template) - 1):
    check = template[i:i+2]
    if check in count_pairs:
        count_pairs[check] += 1
    else:
        count_pairs[check] = 1

count_chars = dict()

for char in template:
    if char in count_chars:
        count_chars[char] += 1
    else:
        count_chars[char] = 1

##print(count_pairs)
for _ in range(40):
    step(count_pairs, count_chars, rules)
##    print(count_pairs)

values = count_chars.values()
highest = max(values)
lowest = min(values)

##print(count_chars)

print(highest - lowest)
