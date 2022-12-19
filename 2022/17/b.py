import itertools
from collections import defaultdict

year = 2022
num = 17
isB = True
test = True

filename = f'{year}/{num}/{"data" if not test else "test"}'
infile = f'{filename}.in'
outfile = f'{filename}_{"a" if not isB else "b"}.out'


pieces = {'-': [(2, 0), (3, 0), (4, 0), (5, 0)],
          '+': [(3, 0), (2, 1), (3, 1), (4, 1), (3, 2)],
          'j': [(2, 0), (3, 0), (4, 0), (4, 1), (4, 2)],
          'l': [(2, 0), (2, 1), (2, 2), (2, 3)],
          'o': [(2, 0), (2, 1), (3, 0), (3, 1)]}


# 2 units from left, 3 units above highest rock
def solver(file):
    order = '-+jlo'
    landed = defaultdict(set)
    moves = list(file)[0]
    print(len(moves))
    max_height = -1

    rocks = [[x, y+max_height+4] for x, y in pieces[order[0]]]
    count = 0
    while count < 1_000_000_000_000:
        for i, move in enumerate(moves):
            if move == '<':
                if all(x!=0 for x, _ in rocks) and all(x-1 not in landed[y] for x, y in rocks):
                    for i in range(len(rocks)):
                        rocks[i][0] -= 1
            else:
                if all(x!=6 for x, _ in rocks) and all(x+1 not in landed[y] for x, y in rocks):
                    for i in range(len(rocks)):
                        rocks[i][0] += 1
            
            if rocks[0][1] == 0 or any(x in landed[y-1] for x, y in rocks):
                for x, y in rocks:
                    landed[y].add(x)
                max_height = max(max_height, rocks[-1][1])
                if i % len(moves) == 0:
                    print(count)
                count += 1
                rocks = [[x, y+max_height+4] for x, y in pieces[order[count%len(pieces)]]]
            else:
                for i in range(len(rocks)):
                    rocks[i][1] -= 1

    return max_height + 1
        


def main():
    with open(infile) as file:
        s = solver(file)

    if s is None:
        return

    print(s)
    with open(outfile, 'w') as file:
        file.write(str(s))


if __name__ == '__main__':
    main()
