year = 2022
num = 15
isB = True
test = False

filename = f'{year}/{num}/{"data" if not test else "test"}'
infile = f'{filename}.in'
outfile = f'{filename}_{"a" if not isB else "b"}.out'


NUM = 20 if test else 4_000_000
dirs = [(1, 1), (1, -1), (-1, 1), (-1, -1)]


def solver(file):
    search = set()
    sensors = set()
    for i, line in enumerate(file):
        line = line.strip().split(' ')
        s1, s2 = int(line[2][2:-1]), int(line[3][2:-1])
        b1, b2 = int(line[-2][2:-1]), int(line[-1][2:])
        
        dist = abs(s1 - b1) + abs(s2 - b2)
        sensors.add((dist, s1, s2))

        for x in range(dist + 2):
            y = dist + 1 - x
            for dx, dy in dirs:
                ax, ay = s1+x*dx, s2+y*dy
                if 0<=ax<=NUM and 0<=ay<=NUM:
                    search.add((ax, ay))
        if i % 5 == 0:
            print(i)
    print(len(sensors))
    print(len(search))
    for i, (x, y) in enumerate(search):
        for dist, a, b in sensors:
            d = abs(a - x) + abs(b - y)
            if d <= dist:
                break
        else:
            return (x, y, x*NUM + y)
        if i % 5_000_000 == 0:
            print(i)


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
