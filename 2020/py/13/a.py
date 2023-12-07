from math import ceil

num = 13
isB = False
test = False

infile = f'{num}/' + ('input.txt' if not test else 'sample.txt')
outfile = f'{num}/{"a" if not isB else "b"}.out.txt'


def solver(file):
    time, buses = list(line.strip() for line in iter(file))
    time = int(time)
    smallest = (-1, time * time)
    for bus in buses.split(','):
        if bus == 'x':
            continue
        bus = int(bus)
        t = bus * ceil(time / bus)
        if t < smallest[1]:
            smallest = (bus, t)
    print(smallest)
    return smallest[0] * (smallest[1] - time)


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
