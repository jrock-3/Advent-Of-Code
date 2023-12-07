num = 2
isB = False
test = False

infile = f'{num}/' + ('input.txt' if not test else 'sample.txt')
outfile = f'{num}/{"a" if not isB else "b"}.out.txt'


def solver(file):
    scores = {'R': 1, 'P': 2, 'S': 3}
    player_to_move = {'A': 'R', 'B': 'P', 'C': 'S',
                      'X': 'R', 'Y': 'P', 'Z': 'S'}
    winning = {'RP', 'PS', 'SR'}  # opponent move, your move
    score = 0

    for line in file:
        line = line.rstrip('\n')
        opp, you = line.split(' ')
        opp, you = player_to_move[opp], player_to_move[you]

        score += scores[you]
        if opp == you:
            score += 3
        if (opp + you) in winning:
            score += 6

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
