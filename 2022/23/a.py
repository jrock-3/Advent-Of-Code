from collections import defaultdict

year = 2022
num = 23
isB = False
test = False

filename = f'{year}/{num}/{"data" if not test else "test"}'
infile = f'{filename}.in'
outfile = f'{filename}_{"a" if not isB else "b"}.out'


def get_ranges(elves):
    minX, minY = min(x for x,_ in elves), min(y for _,y in elves)
    maxX, maxY = max(x for x,_ in elves), max(y for _,y in elves)
    return range(minX,maxX+1), range(minY,maxY+1)


def print_elves(elves):
    xr, yr = get_ranges(elves)
    for i in xr:
        for j in yr:
            print('#' if (i,j) in elves else '.', end='')
        print()
    print()


def solver(file):
    elves = set()
    for i, line in enumerate(file):
        for j, char in enumerate(line):
            if char == '#':
                elves.add((i, j))
    
    # print_elves(elves)
    
    '''
    765
    4X3
    210
    '''
    dirs = [(1, 1), (1, 0), (1, -1), (0, 1), (0, -1), (-1, 1), (-1, 0), (-1, -1)]
    N = (7, 6, 5)
    S = (2, 1, 0)
    W = (7, 4, 2)
    E = (5, 3, 0)
    res_ords = {N: (-1,0), S: (1, 0), W: (0,-1), E: (0,1)}
    ord_letters = {N: 'N', S: 'S', W: 'W', E: 'E'}
    orders = [N,S,W,E]
    for i in range(1, 11):
        # print([ord_letters[ord] for ord in orders])
        proposed = defaultdict(list)
        _elves = set()
        # print(i)
        for x,y in elves:
            
            ds = []
            for dx,dy in dirs:
                ds.append((x+dx, y+dy) in elves)
            
            dx, dy = 0, 0
            if any(d for d in ds):
                for order in orders:
                    if all(not ds[i] for i in order):
                        dx, dy = res_ords[order]
                        break
            if not (dx == dy == 0):
                elf = (x+dx, y+dy)
                proposed[elf].append((x,y))
            else:
                # print((x,y), (x,y))
                _elves.add((x,y))
        for elf, elfs in proposed.items():
            if len(elfs) > 1:
                for e in elfs:
                    _elves.add(e)
            else:
                # print(elfs[0], elf)
                _elves.add(elf)
        elves = _elves
        # print_elves(elves)
        orders.append(orders.pop(0))
        # return
    
    xr, yr = get_ranges(elves)
    count = 0
    for i in xr:
        for j in yr:
            if (i,j) not in elves:
                count += 1
    return count


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
