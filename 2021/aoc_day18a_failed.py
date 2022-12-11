# aoc_day18a_failed.py

import json, copy

with open('aoc_day18_test.txt') as file:
    data = file.read().split()

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

def replace_split(l):
    a, b = l
    for i, x in enumerate((a, b)):
        if type(x) == int:
            if x >= 10:
                x = split(x)
        else:
            x = replace_split(x)
        if i == 0:
            a = x
        else:
            b = x
    return [a, b]

def check_split(l):
    try:
        ll = json.loads(l)
    except json.decoder.JSONDecodeError:
        print(l)

    ll = replace_split(ll)

    return str(ll)

def check_explode(l):
    count = 0
    idx = 0
    while idx < len(l) and count < 5:
        if l[idx] == '[':
            count += 1
        elif l[idx] == ']':
            count -= 1
        idx += 1
    if count != 5:
        return l
##    print(l[idx])
    if l[idx] == '[':
        idx += 1
    a = l.index(',',idx)+2
##    print(l[a-2:a+1])
    if l[a] == '[':
##        print('hi')
        idx = a+1
##    print(l[:idx])
##    print(l[idx:])

    try:
        i = l.index(']',idx)
    except ValueError:
        return l
    first = l[:idx-1]
##    print(first)
    explode = l[idx-1:i+1].strip('][').split(',')
    last = l[i+1:]
##    print(explode)

    idx = len(first)-1
    middle = ''
    while idx >= 0 and not first[idx].isdigit():
        middle = first[idx] + middle
        idx -= 1
    if idx > 0 and first[idx].isdigit():
        num = ''
        while idx >= 0 and first[idx].isdigit():
            num = first[idx] + num
            idx -= 1
        
        num = int(num)
        num += int(explode[0])
        first = first[:idx+1] + str(num) + middle
    first += '0'

    idx = 0
    middle = ''
    while idx < len(last) and not last[idx].isdigit():
        middle += last[idx]
        idx += 1
    _idx = idx
    if idx < len(last) and last[idx].isdigit():
        num = ''
        while idx < len(last) and last[idx].isdigit():
            num += last[idx]
            idx += 1
        num = int(num)
        num += int(explode[1])
        last = middle + str(num) + last[idx:]
    return first + last

def add(t, l):
    l = f'[{t}, {l}]'
    print(l)
    print()
    _l = ''

    while l != _l:
        _l = copy.deepcopy(l)

        l = check_split(l)
        print(l)
        l = check_explode(l)
        print(l)
        print()
    return l

t = data.pop(0)
for l in data:
##    print(t)
##    print(l)
    t = add(t, l)
    print('\n')
##    print(t)

print(t)

t = json.loads(t)

def do_sum(line):
    a, b = line
    if type(a) == int and type(b) == int:
        return 3*a + 2*b
    if type(a) == int:
        return 3*a + 2*do_sum(b)
    if type(b) == int:
        return 3*b + 2*do_sum(a)
    return 3*do_sum(a) + 2*do_sum(b)

print(do_sum(t))
