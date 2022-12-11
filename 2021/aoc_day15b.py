# aoc_day15b.py (doesn't work)

def valid(risks, i, j):
    return 0<=i<len(risks) and 0<=j<len(risks[0])

def set_risks(risks, x, y):
    z = 999999999999999999
    if valid(risks, x-1, y):
        z = min(z, risks[x-1][y])
    if valid(risks, x, y-1):
        z = min(z, risks[x][y-1])
    risks[x][y] += z

 
with open('aoc_day15.txt') as file:
    data = file.read().split('\n')

risks = [[int(i) for i in list(r)] for r in data]


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

risks[0][0] = 0


for i in range(1, len(risks)):
    for x in range(i, -1, -1):
        y = i - x
        set_risks(risks, x, y)

for i in range(1, len(risks)):
    for x in range(i, len(risks)):
        y = len(risks) - x + i - 1
        set_risks(risks, x, y)

print(risks[len(risks)-1][len(risks)-1])
