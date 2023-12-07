num = 7
isB = True
test = False

infile = f'{num}/' + ('input.txt' if not test else 'sample.txt')
outfile = f'{num}/{"a" if not isB else "b"}.out.txt'


class Dir:
    def __init__(self, name, parent):
        self.name = name
        self.parent = parent
        self.total = 0
        self.dirs = set()

    def __repr__(self):
        return f'(name: {self.name}\nparent: {"" if self.parent is None is None else self.parent.name}\ntotal: {self.total}\ndirs: {[dir.name for dir in self.dirs]})\n'


def calc_total(dir):
    total = dir.total
    dirs = dir.dirs
    if len(dirs) == 0:
        return total
    for d in dirs:
        total += calc_total(d)
    return total


def solver(file):
    root = Dir('/', None)
    curDir = root
    dirs = {'/': curDir}

    # get rid of line 1
    file = iter(file)
    next(file)

    for line in file:
        line = line.rstrip('\n').split(' ')

        # cmds
        if line[0] == '$':
            if line[1] == 'ls':
                continue
            if line[2] == '..':
                curDir = curDir.parent
                continue
            if line[2] not in dirs:
                dirs[line[2]] = Dir(line[2], curDir)
            curDir.dirs.add(dirs[line[2]])
            curDir = dirs[line[2]]
            continue

        # ls outputs
        if line[0] == 'dir':
            dirs[line[1]] = Dir(line[1], curDir)
            curDir.dirs.add(dirs[line[1]])
            continue

        curDir.total += int(line[0])

    total = 0
    for dir in dirs.values():
        calc_sum = calc_total(dir)
        if calc_sum <= 100_000:
            total += calc_sum

    return total


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
