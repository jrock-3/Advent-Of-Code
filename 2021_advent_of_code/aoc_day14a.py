# aoc_day14a.py

def step(template, rules):
    new = ''
    for i in range(len(template) - 1):
        check = template[i:i+2]
        if check in rules:
            new += check[0] + rules[check]
        else:
            new += check[0]

    new += template[-1]

    return new

with open('aoc_day14.txt') as file:
    data = file.read().split('\n')

template = data.pop(0)
data.pop(0)

rules = dict()

for line in data:
    pair, insert = line.split(' -> ')
    rules[pair] = insert

for _ in range(10):
    template = step(template, rules)

counts = dict()
for char in template:
    if char in counts:
        counts[char] += 1
    else:
        counts[char] = 1

highest = 0
lowest = len(template)
for key, value in counts.items():
    if value > highest:
        highest = value
    elif value < lowest:
        lowest = value

print(highest - lowest)
