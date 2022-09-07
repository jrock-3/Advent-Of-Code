# 20_7a.py

def find_bags(rules, search):
    total = 0
    for bag in rules[search]:
        total += bag[1] + bag[1] * find_bags(rules, bag[0])

    return total

rules = dict()

with open('20_7.txt') as file:
    line = file.readline().strip('\n')

    while line != '':
        bag, others = line.split(' contain ')

        bag = bag.rstrip('s ')

        if others == 'no other bags.':
            rules[bag] = list()
            line = file.readline().strip('\n')
            continue

        bags = list()
        for other in others.split(', '):
            bags.append((other[1:].strip('. ').rstrip('s'), int(other[0])))

        rules[bag] = bags

        line = file.readline().strip('\n')

print(find_bags(rules, 'shiny gold bag'))
