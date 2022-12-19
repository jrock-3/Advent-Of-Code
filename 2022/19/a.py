# taken from https://github.com/jonathanpaulson/AdventOfCode/blob/master/2022/19.py

from collections import deque

year = 2022
num = 19
isB = False
test = False

filename = f'{year}/{num}/{"data" if not test else "test"}'
infile = f'{filename}.in'
bestfile = f'{filename}_{"a" if not isB else "b"}.best'


# def simulate(blueprint, t, robots, resources):
#     if t == 0:
#         return resources[3]
#     resources = [x+y for x,y in zip(resources, robots)]
#     t -= 1
#     best = simulate(blueprint, t, robots, resources)
#     for cost in blueprint:
#         test = [x-y for x, y in zip(resources, cost)]
#         if min(test) >= 0:
#             best = max(best, simulate(blueprint, t-1, robots, test))
#     return best


# start with one ore collecting robot
def solver(file):
    total = 0
    for i, line in enumerate(file):
        line = line.strip().split(':')[-1].strip().split()
        
        # costs ore
        c1 = int(line[4])
        # costs ore
        c2 = int(line[10])
        # costs, ore, clay
        c3 = (int(line[16]), int(line[19]))
        # costs ore, obsidian
        c4 = (int(line[25]), int(line[28]))

        m1 = max(c1, c2, c3[0], c4[0])
        m2 = c3[1]
        m3 = c4[1]

        # print(c1, c2, c3, c4, m1, m2, m3)

        # time, ore, clay, obsidian, geode, oreR, clayR, obsidianR, geodeR
        best = 0
        S = (0, 0, 0, 0, 1, 0, 0, 0, 24)
        Q = deque([S])
        SEEN = set()
        while Q:
            o, c, ob, g, r1, r2, r3, r4, t = Q.popleft()
            
            best = max(best, g)
            if t == 0:
                continue

            r1 = min(r1, m1)
            r2 = min(r2, m2)
            r3 = min(r3, m3)
            o = min(o, t*m1 - r1*(t-1))
            c = min(c, t*m2 - r2*(t-1))
            ob = min(ob, t*m3 - r3*(t-1))
            
            state = (o, c, ob, g, r1, r2, r3, r4, t)
            if state in SEEN:
                continue
            SEEN.add(state)

            Q.append((o+r1, c+r2, ob+r3, g+r4, r1, r2, r3, r4, t-1))
            if o >= c1:
                Q.append((o+r1-c1, c+r2, ob+r3, g+r4, r1+1, r2, r3, r4, t-1))
            if o >= c2:
                Q.append((o+r1-c2, c+r2, ob+r3, g+r4, r1, r2+1, r3, r4, t-1))
            if o >= c3[0] and c >= c3[1]:
                Q.append((o+r1-c3[0], c+r2-c3[1], ob+r3, g+r4, r1, r2, r3+1, r4, t-1))
            if o >= c4[0] and ob >= c4[1]:
                Q.append((o+r1-c4[0], c+r2, ob+r3-c4[1], g+r4, r1, r2, r3, r4+1, t-1))
        
        print(i+1, best)
        total += (i+1) * best
    return total


def main():
    with open(infile) as file:
        s = solver(file)

    if s is None:
        return

    print(s)
    with open(bestfile, 'w') as file:
        file.write(str(s))


if __name__ == '__main__':
    main()
