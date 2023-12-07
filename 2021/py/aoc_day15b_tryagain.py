# aoc_day15b_tryagain.py

# read input
with open('aoc_day15.txt') as file:
    data = file.read().split('\n')
###

# format input
risks = [[int(i) for i in list(r)] for r in data]

#extra part for part b
for i, line in enumerate(risks):
    temp = line[::]
    for x in range(4):
        a = [(e + x)%9+1 for e in line]
        temp.extend(a)
    risks[i] = temp

temp = list()
for x in range(4):
    a = [[(e + x)%9+1 for e in line] for line in risks]
    temp.extend(a)
risks.extend(temp)
##

risks[0][0] = 0
###

# solve
def valid(_max, i, j):
    return 0<=i<_max and 0<=j<_max
largest = sum([sum(r) for r in risks])
LW = len(risks)

heap = [(0, 0)]

dist = list()
for i in range(LW):
    temp = list()
    for j in range(LW):
        temp.append(largest)
    dist.append(temp)
dist[0][0] = 0

while len(heap) > 0:
    v = heap.pop(0)
    for dx, dy in ( (1,0), (-1,0), (0,1), (0,-1) ):
        x = v[0]+dx
        y = v[1]+dy
        if not valid(LW,x,y):
            continue
        check = dist[v[0]][v[1]] + risks[x][y]
        if dist[x][y] > check:
            dist[x][y] = check
            heap.append((x,y))

print(dist[LW-1][LW-1])
###
