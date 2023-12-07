# aoc_day18a_alsofailed.py

import json, copy

with open('aoc_day18_test.txt') as file:
    data = [json.loads(l) for l in file.read().split()]

##print(data)

def findnth(string, substring, n):
    parts = string.split(substring, n + 1)
    if len(parts) <= n + 1:
        return -1
    return len(string) - len(parts[-1]) - len(substring)

def split(num):
    a = num // 2
    b = num - a
    return [a, b]

def process(l, depth):
    if depth == 4:
        return explode(l)
    a, b = l
    for i, x in enumerate((a, b)):
        if type(x) == int:
            if x >= 10:
                x = split(x)
        else:
            x = process(x, depth+1)
        if i == 0:
            a = x
        else:
            b = x

def add(t, l):
    l = process(l, 0)
    return [t, l]

t = data.pop(0)
for l in data:
    t = add(t, l)
    print(t)
    assert False

print(do_sum(t))

def do_sum(line):
    a, b = line
    if type(a) == int and type(b) == int:
        return 3*a + 2*b
    if type(a) == int:
        return 3*a + 2*do_sum(b)
    if type(b) == int:
        return 3*b + 2*do_sum(a)
    return 3*do_sum(a) + 2*do_sum(b)
