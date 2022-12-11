num = 8
isB = False
test = False

infile = f'{num}/' + ('input.txt' if not test else 'sample.txt')
outfile = f'{num}/{"a" if not isB else "b"}.out.txt'


def solver(file):
    dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)]
    trees = []
    for line in file:
        trees.append([int(c) for c in line.strip()])
    inner = 0
    for i in range(1, len(trees) - 1):
        for j in range(1, len(trees[0]) - 1):
            for dx, dy in dirs:
                arr = [trees[i+dx*x][j+dy*x] for x in range(max(len(trees), len(
                    trees[0]))) if 0 <= i+dx*x < len(trees) and 0 <= j+dy*x < len(trees[0])]
                if trees[i][j] == max(arr) and arr.count(trees[i][j]) == 1:
                    inner += 1
                    break
    outside = 2 * (len(trees) + len(trees[0])) - 4
    return outside + inner


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
