# aoc_day5a.py

board = list()

for i in range(999):
    board.append([0] * 999)


def add_line(one, two):
    x1, y1 = one
    x2, y2 = two

    if abs(x1-x2) == abs(y1-y2):
        for i in range(abs(x2 - x1)+1):
            dx = x1-i if x1-x2>0 else x1+i
            dy = y1-i if y1-y2>0 else y1+i
            board[dx][dy] += 1
        return
    elif not(x1 == x2 or y1 == y2):
        return

    if x1 > x2:
        x1, x2 = x2, x1
    if y1 > y2:
        y1, y2 = y2, y1

    for i in range(x1, x2+1):
        for j in range(y1, y2+1):
            board[i][j] += 1


with open('aoc_day5.txt') as file:
    line = file.readline()

    while line != '':
        ones, _, twos = line.split()
        x1, y1 = ones.split(',')
        x2, y2 = twos.split(',')

        add_line((int(x1), int(y1)), (int(x2), int(y2)))

        line = file.readline()

count = 0
for row in board:
    for item in row:
        if item > 1:
            count += 1

print(count)
