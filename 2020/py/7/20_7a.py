# 20_7a.py

def find_starts(rules, search):
##    print(f'---searching for {search}---')
    
    possible = set()

    for rule, bags in rules.items():
        if search in [bag[0] for bag in bags]:
##            print(f'{search:>15}: {rule}')
            possible.add(rule)
            possible |= find_starts(rules, rule)

##    print(f'---done searching for {search}---')

    return possible

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

##for key, value in rules.items():
##    print(f'{key:>20}:{value}')

all_starts = find_starts(rules, 'shiny gold bag')

##for start in all_starts:
##    print(start)

print(len(all_starts))
