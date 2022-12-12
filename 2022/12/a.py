year = 2022
num = 12
isB = False
test = False

filename = f'{year}/{num}/{"data" if not test else "test"}'
infile = f'{filename}.in'
outfile = f'{filename}_{"a" if not isB else "b"}.out'


def is_legal(heights, source, curr):
    sz_rows = len(heights)
    sz_cols = len(heights[0])
    x, y = source
    a, b = curr
    in_bounds = 0 <= a < sz_rows and 0 <= b < sz_cols
    if not in_bounds:
        return False
    return heights[a][b] - heights[x][y] <= 1


def print_visited(sz_row, sz_col, ds):
    for i in range(sz_row):
        for j in range(sz_col):
            if (i, j) in ds:
                print('x', end='')
            else:
                print('.', end='')
        print()
    print()


def solver(file):
    heights = []
    start, end = (-1, -1), (-1, -1)
    for i, line in enumerate(file):
        heights.append([])
        for j, char in enumerate(line.strip()):
            if char == 'S':
                start = (i, j)
                heights[i].append(1)
            elif char == 'E':
                end = (i, j)
                heights[i].append(26)
            else:
                heights[i].append(ord(char) - 96)
    # print(heights, start, end)

    # for row in heights:
    #     for num in row:
    #         print(f'{chr(num+96):>3}', end='')
    #     print()

    # bfs
    dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)]
    fq = [start]
    ds = {start}
    levels = dict()
    levels[start] = 0
    visited = set()
    parents = {start: None}
    while len(fq) > 0:
        curr = fq.pop(0)
        # visit
        visited.add(curr)
        # print(f'visiting {curr}')
        # print_visited(len(heights), len(heights[0]), visited)
        if curr == end:
            print(curr, levels[curr])
            # x = curr
            # path = []
            # while x is not None:
            #     path.append(x)
            #     print(x, heights[x[0]][x[1]])
            #     x = parents[x]
            # path = path[::-1]
            # for i in range(1, len(path)):
            #     for a in range(len(heights)):
            #         for b in range(len(heights[0])):
            #             if (a, b) in path[:i]:
            #                 print('x', end='')
            #             else:
            #                 print('.', end='')
            #         print()
            #     print()
            return
        for dx, dy in dirs:
            to = (curr[0]+dx,curr[1]+dy)
            if to not in ds and is_legal(heights, curr, to):
                fq.append(to)
                ds.add(to)
                levels[to] = levels[curr] + 1
                parents[to] = curr
    # print(levels)
    # for k,v in levels.items():
    #     x, y = k
    #     if heights[x][y] == 0:
    #         print(k, v)


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
