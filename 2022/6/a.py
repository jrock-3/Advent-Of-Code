num = 6
isB = False
test = False

infile = f'{num}/' + ('input.txt' if not test else 'sample.txt')
outfile = f'{num}/{"a" if not isB else "b"}.out.txt'


def solver(file):
    line = next(iter(file))
    for i in range(len(line)-4):
        chars = line[i:i+4]
        if len(set(chars)) == 4:
            return i + 4
    return -1


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
