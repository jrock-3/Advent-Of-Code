# 20_4a.py

fields = {'byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl','pid'}# ,'cid']
total = 0

with open('20_4.txt') as file:
    data = file.read()
    lines = [[i.split(':') for i in x.replace('\n', ' ').split()] for x in data.split('\n\n')]
    for line in lines:
        if {x[0] for x in line} - {'cid'} == fields:
            total += 1

print(total)
