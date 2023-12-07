# aoc_day12a.py

import copy

def do(paths, start, visited, sofar):
    if start == 'dc':
        print(sofar, visited)
    _visited = copy.deepcopy(visited)
    _sofar = copy.deepcopy(sofar)
    _sofar.append(start)
    if start == 'end':
##        print(_sofar)
        return 1
    
    if not _visited[start] and start.upper() != start:
##        print('.', start)
        _visited[start] = True

    t = 0
    for path in paths:
        if path[0] == start and not _visited[path[1]]:
##            print(start, path)
            t += do(paths, path[1], _visited, _sofar)
        if path[1] == start and not _visited[path[0]]:
##            print(start, path)
            t += do(paths, path[0], _visited, _sofar)
    return t

paths = list()
starters = list()

with open('aoc_day12.txt') as file:
    data = file.read().split('\n')

for line in data:
    start, end = line.split('-')
    if start != 'start' and end != 'start':
        paths.append((start, end))
    elif start == 'start':
        starters.append((start, end))
    elif end == 'start':
        starters.append((end, start))

##print(paths)
##print(starters)

visited = dict()

for path in paths:
    visited[path[0]] = False
    visited[path[1]] = False

total = 0
for start in starters:
    total += do(paths, start[1], visited, ['start'])

print(total)
