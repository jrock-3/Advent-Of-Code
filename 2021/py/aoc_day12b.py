# aoc_day12b.py

import copy

def do(paths, start, visited, sofar):
    _visited = copy.deepcopy(visited)
    _sofar = copy.deepcopy(sofar)
    _sofar.append(start)
    if start == 'end':
        return 1
    
    if _visited[start] < 2 and start.upper() != start:
        _visited[start] += 1

    limit = 2
    if 2 in [y for x, y in _visited.items()]:
        limit = 1

    t = 0
    for path in paths:
        if path[0] == start and _visited[path[1]] < limit:
            t += do(paths, path[1], _visited, _sofar)
        if path[1] == start and _visited[path[0]] < limit:
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

print(paths)
print(starters)

visited = dict()

for path in paths:
    visited[path[0]] = 0
    visited[path[1]] = 0

total = 0
for start in starters:
    total += do(paths, start[1], visited, ['start'])

print(total)
