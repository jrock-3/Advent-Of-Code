import sys

year = 2022
num = 18
isB = True
test = False

filename = f'{year}/{num}/{"data" if not test else "test"}'
infile = f'{filename}.in'
outfile = f'{filename}_{"a" if not isB else "b"}.out'


# sys.setrecursionlimit(4000)

adj = [(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)]


def dfs(cubes, ranges, outer, coord):
    if coord not in cubes:
        outer.add(coord)
    xr, yr, zr = ranges
    x, y, z = coord
    for dx, dy, dz in adj:
        _coord = (x+dx, y+dy, z+dz)
        _x, _y, _z = _coord
        is_legal = _x in xr and _y in yr and _z in zr
        not_cube = _coord not in cubes
        not_visited = _coord not in outer
        if is_legal and not_cube and not_visited:
            dfs(cubes, ranges, outer, _coord)


def solver(file):
    cubes = {tuple(map(int, line.strip().split(','))) for line in file}
    
    mins = (min(x for x,_,_ in cubes), min(y for _,y,_ in cubes), min(z for _,_,z in cubes))
    maxes = (max(x for x,_,_ in cubes), max(y for _,y,_ in cubes), max(z for _,_,z in cubes))
    xr,yr,zr = tuple(range(x-1, y+2) for x, y in zip(mins, maxes))

    print(mins, maxes)

    # outer = set()
    # dfs(cubes, ranges, outer, mins)

    stack = [tuple(x-1 for x in mins)]
    visited = set()
    while len(stack) > 0:
        x,y,z = stack.pop()
        visited.add((x,y,z))
        for dx,dy,dz in adj:
            nx,ny,nz = x+dx,y+dy,z+dz
            if nx in xr and ny in yr and nz in zr and (nx,ny,nz) not in visited and (nx,ny,nz) not in cubes:
                stack.append((nx,ny,nz))
    
    # print(visited)

    count = 0
    for x, y, z in cubes:
        for dx, dy, dz in adj:
            neighbor = (x+dx, y+dy, z+dz)
            if neighbor not in cubes and neighbor in visited:
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
