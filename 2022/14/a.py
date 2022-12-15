year = 2022
num = 14
isB = False
test = False

filename = f'{year}/{num}/{"data" if not test else "test"}'
infile = f'{filename}.in'
outfile = f'{filename}_{"a" if not isB else "b"}.out'


NUM = 1000


def legal_move(board, x, y):
    return 0 <= x < NUM and 0 <= y < NUM and board[x][y] == '.'


def drop(board, x, y):
    if legal_move(board, x, y+1):
        return x, y+1
    if legal_move(board, x-1, y+1):
        return x-1, y+1
    if legal_move(board, x+1, y+1):
        return x+1, y+1
    return x, y


def print_board(board, x1, x2, y1, y2): #494, 503, 0, 9)
    for j in range(y1, y2+1):
        for i in range(x1, x2+1):
            print(board[i][j], end='')
        print()
    print()



def solver(file):
    max_val = 0
    board = [['.' for _ in range(NUM)] for _ in range(NUM)]
    for line in file:
        prev = None
        for coord in line.strip().split(' -> '):
            x, y = map(int, coord.split(','))
            max_val = max(max_val, y)
            if prev == None:
                # print(x, y)
                board[x][y] = '#'
            else:
                a, b = prev
                if x - a == 0:
                    # change y
                    for i in range(min(b, y), max(b, y) + 1):
                        # print(x, i)
                        board[x][i] = '#'
                else:
                    # change x
                    for i in range(min(a, x), max(a, x) + 1):
                        # print(i, y)
                        board[i][y] = '#'
            prev = (x, y)
    
    # print(max_val)
    for i in range(NUM):
        board[i][max_val+2] = '#'
    # print_board(board, 490, 510, 0, 11)

    start = (500, 0)
    count = 0
    while True:
        x, y = start
        _x, _y = start
        # print(count)
        while True:
            x, y = drop(board, x, y)
            if (x, y) == (_x, _y):
                break
            _x, _y = x, y
        count += 1
        if (x, y) == start:
            break
        # print(x, y)
        board[x][y] = 'o'
        # print_board(board, 490, 510, 0, 11)
    return count


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
