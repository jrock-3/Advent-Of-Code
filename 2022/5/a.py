num = 5
isB = False
test = False

infile = f'{num}/' + ('input.txt' if not test else 'sample.txt')
outfile = f'{num}/{"a" if not isB else "b"}.out.txt'


def solver(file):
    stacks = []
    for line in file:
        if line == '\n':
            stacks = [e[::-1] for e in stacks]
            continue
        if line.startswith('move'):
            num, from_col, to_col = [
                int(x) for x in line.rstrip('\n').split(' ')[1::2]]
            for _ in range(num):
                stacks[to_col-1].append(stacks[from_col-1].pop())
            continue
        line = line.rstrip('\n')[1::4]
        if line.isnumeric():
            continue
        for i, char in enumerate(line):
            if len(stacks) <= i:
                stacks.append([])
            if char != ' ':
                stacks[i].append(char)
    return ''.join(s.pop() for s in stacks)


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
