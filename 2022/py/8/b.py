num = 8
isB = True
test = False

infile = f'{num}/' + ('input.txt' if not test else 'sample.txt')
outfile = f'{num}/{"a" if not isB else "b"}.out.txt'


def solver(file):
    dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)]
    trees = []
    for line in file:
        trees.append([int(c) for c in line.strip()])
    scores = []
    for i in range(1, len(trees) - 1):
        for j in range(1, len(trees[0]) - 1):
            score = 1
            for dx, dy in dirs:
                arr = []
                for x in range(1, max(len(trees), len(trees[0]))):
                    if 0 <= i+dx*x < len(trees) and 0 <= j+dy*x < len(trees[0]):
                        val = trees[i+dx*x][j+dy*x]
                        arr.append(val)
                        if val >= trees[i][j]:
                            break
                score *= len(arr)
            scores.append(score)
    return max(scores)


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
