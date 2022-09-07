# aoc_day4b.py

import copy

def check_vertical(board):
    for i in range(len(board[0])):
        if False not in [board[j][i] == 'XX' for j in range(len(board))]:
            return True
    return False

def check_horizontal(board):
    for row in board:
        if False not in [item == 'XX' for item in row]:
            return True
    return False
        
def update_boards(boards, num):
    for i, board in enumerate(boards):
        if type(board) != list:
            continue
        for j, row in enumerate(board):
            for k, item in enumerate(row):
                if item == num:
                    boards[i][j][k] = 'XX'

def get_unmarked_total(board):
    total = 0
    for row in board:
        for item in row:
            if item != 'XX':
                total += int(item)
    return total

def print_boards(boards):
    for board in boards:
        for row in board:
            for element in row:
                print(f'{element:>3}', end='')
            print()
        print('\n')


with open('aoc_day4.txt') as file:
    draws = file.readline().strip('\n').split(',')

    line = file.readline()

    boards = list()
    i = 0
    while line != '':
        boards.append(list())
        
        for _ in range(5):
            boards[i].append(file.readline().strip('\n').split())
        line = file.readline()
        i += 1

backup_draws = copy.deepcopy(draws)
backup_boards = copy.deepcopy(boards)

x = 0
for num in draws:
    update_boards(boards, num)

    for i, board in enumerate(boards):
        if type(board) == list and (check_vertical(board) or check_horizontal(board)):
            boards[i] = x
            x += 1

last_num = 0
idx = 0
for i, board in enumerate(boards):
    if type(board) == int:
        if last_num < board:
            last_num = board
            idx = i


for num in backup_draws:
    update_boards(backup_boards, num)

    for i, board in enumerate(backup_boards):
        if i == idx and (check_vertical(board) or check_horizontal(board)):
            unmarked_total = get_unmarked_total(board)
            print(int(num) * unmarked_total)
            assert False
