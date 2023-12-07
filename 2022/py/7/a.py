# https://www.youtube.com/watch?v=ZPM5xclRInk

from collections import defaultdict

num = 7
isB = False
test = False

infile = f'{num}/' + ('input.txt' if not test else 'sample.txt')
outfile = f'{num}/{"a" if not isB else "b"}.out.txt'


def solver(file):
    path = []
    sizes = defaultdict(int)
    for line in file:
        words = line.rstrip('\n').split(' ')
        if words[1] == 'cd':
            if words[2] == '..':
                path.pop()
            else:
                path.append(words[2])
        elif words[0].isnumeric():
            size = int(words[0])
            for i in range(len(path)+1):
                sizes[''.join(path[:i])] += size
    total = 0
    for v in sizes.values():
        if v <= 100_000:
            total += v
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
