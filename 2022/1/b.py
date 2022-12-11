num = 1
isB = True
file_name = f'{num}.txt'
test = False


def solve():
    with open(f'{num}/{file_name if not test else "sample.txt"}') as file:
        write_answer(solver(file))


def solver(file):
    maxelfs = []
    total = 0

    for line in file:
        if line == '\n':
            maxelfs.append(total)
            total = 0
        else:
            total += int(line)

    if total != 0:
        maxelfs.append(total)

    maxelfs = sorted(maxelfs)[-3:]

    return (maxelfs, sum(maxelfs))


def write_answer(s):
    print(s)
    with open(f'{num}/{"a" if not isB else "b"}.out.txt', 'w') as file:
        file.write(str(s))


def main():
    solve()


if __name__ == '__main__':
    main()
