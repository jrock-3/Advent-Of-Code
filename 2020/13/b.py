from math import ceil

num = 13
isB = True
test = True

infile = f'{num}/' + ('input.txt' if not test else 'sample.txt')
outfile = f'{num}/{"a" if not isB else "b"}.out.txt'


def solver(file):
    _, buses = list(line.strip() for line in iter(file))
    buses = [x if x == 'x' else int(x) for x in buses.split(',')]
    for i, bus in enumerate(buses):
        pass


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
