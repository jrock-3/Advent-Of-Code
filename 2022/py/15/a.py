year = 2022
num = 15
isB = False
test = False

filename = f'{year}/{num}/{"data" if not test else "test"}'
infile = f'{filename}.in'
outfile = f'{filename}_{"a" if not isB else "b"}.out'


NUM = 10 if test else 2_000_000


def solver(file):
    out = set()
    beacons = set()
    for line in file:
        line = line.strip().split(' ')
        s1, s2 = int(line[2][2:-1]), int(line[3][2:-1])
        b1, b2 = int(line[-2][2:-1]), int(line[-1][2:])
        if b2 == NUM:
            beacons.add(b1)

        dist = abs(s1 - b1) + abs(s2 - b2)
        x = dist - abs(s2 - NUM)
        if x >= 0:
            for i in range(-x, x+1):
                out.add(s1 + i)
    return len(out - beacons)


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
