year = 2022
num = 13
isB = False
test = False

filename = f'{year}/{num}/{"data" if not test else "test"}'
infile = f'{filename}.in'
outfile = f'{filename}_{"a" if not isB else "b"}.out'


def compare(a, b, debug):
    is_a = isinstance(a, int)
    is_b = isinstance(b, int)
    if debug:
        print(a, b)
    if is_a and is_b:
        if a == b:
            return None
        return a < b
    elif is_a and not is_b:
        return compare([a], b, debug)
    elif not is_a and is_b:
        return compare(a, [b], debug)
    elif not is_a and not is_b:
        for x, y in zip(a, b):
            val = compare(x, y, debug)
            if val is not None:
                return val
        if len(a) == len(b):
            return None
        return len(a) < len(b)


def solver(file):
    lines = []
    arr = []
    for line in file:
        if line == '\n':
            lines.append(arr)
            arr = []
        else:
            arr.append(eval(line.strip()))
    lines.append(arr)

    out = 0
    for i, (a, b) in enumerate(lines, 1):
        if compare(a, b, False):
            out += i
    return out


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
