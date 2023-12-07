year = 2022
num = 18
isB = False
test = False

filename = f'{year}/{num}/{"data" if not test else "test"}'
infile = f'{filename}.in'
outfile = f'{filename}_{"a" if not isB else "b"}.out'


adj = [(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)]


def solver(file):
    cubes = {tuple(map(int, line.strip().split(','))) for line in file}
    count = 0
    for x, y, z in cubes:
        for dx, dy, dz in adj:
            neighbor = (x+dx, y+dy, z+dz)
            if neighbor not in cubes:
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
