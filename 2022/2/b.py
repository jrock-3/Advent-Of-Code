num = 2
isB = True
test = False

infile = f'{num}/' + ('input.txt' if not test else 'sample.txt')
outfile = f'{num}/{"a" if not isB else "b"}.out.txt'


def solver(file):
    scores = {'A': 1, 'B': 2, 'C': 3}
    wins = {'A': 'B', 'B': 'C', 'C': 'A'}
    loses = {v: k for k, v in wins.items()}
    score = 0

    for line in file:
        line = line.rstrip('\n')
        opp, you = line.split(' ')

        if you == 'Z':
            you = wins[opp]
            score += 6
        elif you == 'X':
            you = loses[opp]
        else:
            you = opp
            score += 3

        score += scores[you]

    return score


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
