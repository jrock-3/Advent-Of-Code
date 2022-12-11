num = 4
isB = False
test = False

infile = f'{num}/' + ('input.txt' if not test else 'sample.txt')
outfile = f'{num}/{"a" if not isB else "b"}.out.txt'


def solver(file):
    count = 0
    for line in file:
        a, b = line.rstrip('\n').split(',')
        x, y = a.split('-')
        z, w = b.split('-')
        x, y, z, w = int(x), int(y), int(z), int(w)
        if x <= z <= y and x <= w <= y or z <= x <= w and z <= y <= w:
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
