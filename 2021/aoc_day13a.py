# aoc_day13a.py

import copy

def _fold(dot, fold):
    diff = abs(fold - dot)
    if dot > fold:
        return fold - diff
    else:
        return dot

with open('aoc_day13.txt') as file:
    data = file.read().split('\n')

dots = set()
folds = list()

trip = False
for line in data:
    if line == '':
        trip = True
        continue

    if trip:
        direction = line.strip('fold along ')
        axis, val = direction.split('=')
        val = int(val)
        folds.append([axis, val])
    else:
        x, y = line.split(',')
        dots.add((int(x), int(y)))

##_dots = list(dots)
##
##_dots.sort()
##print(_dots)

##print(dots)
##print(folds)

for fold in folds:
    for dot in copy.deepcopy(dots):
        dots.remove(dot)
        
        if fold[0] == 'x':
            new = (_fold(dot[0], fold[1]), dot[1])
        else:
            new = (dot[0], _fold(dot[1], fold[1]))

        dots.add(new)
    break

##_dots = list(dots)
##
##_dots.sort()
##print(_dots)

print(len(dots))
