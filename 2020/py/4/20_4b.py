# 20_4a.py

def all(line):
    count = 0
    for key, value in line:
        if key == 'byr' and 1920 <= int(value) <= 2002:
            count += 1
        if key == 'iyr' and 2010 <= int(value) <= 2020:
            count += 1
        if key == 'eyr' and 2020 <= int(value) <= 2030:
            count += 1
        if key == 'hgt':
            num, unit = int(value[:-2]), value[-2:]
            if unit == 'cm' and 150 <= num <= 193:
                count += 1
            elif unit == 'in' and 59 <= num <= 76:
                count += 1
        if key == 'hcl' and value[0] == '#':
            for char in list(value[1:]):
                if char not in 'abcdef0123456789':
                    return False
            count += 1
        if key == 'ecl' and value in 'amb blu brn gry grn hzl oth'.split():
            count += 1
        if key == 'pid':
            try:
                int(value)
            except ValueError:
                return False
            if len(value) == 9:
                count += 1
    return count == len(fields)

def all_valid(line):
    try:
##        print(line)
        return all(line)
    except ValueError:
        return False

fields = {'byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl','pid'}# ,'cid']
total = 0

with open('20_4.txt') as file:
    data = file.read()
    lines = [[i.split(':') for i in x.replace('\n', ' ').split()] for x in data.split('\n\n')]
    for line in lines:
        if {x[0] for x in line} - {'cid'} == fields and all_valid(line):
            total += 1
##        print(total)
##        assert False

print(total)
