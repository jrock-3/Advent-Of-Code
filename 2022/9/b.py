NUM = 999

def move(head, tail):
    dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)]
    
    hx, hy = head
    tx, ty = tail

    # move diagonally
    if hx != tx and hy != ty and (abs(hx - tx) > 1 or abs(hy - ty) > 1):
        dx = 1 if hx - tx > 0 else -1
        dy = 1 if hy - ty > 0 else -1
        # print(dx, dy, 'd')
        return dx, dy
    for dx, dy in dirs:
        if hx == tx + dx*2 and hy == ty + dy*2:
            # print(dx, dy)
            return dx, dy
    return 0, 0

def print_board(head, tails):
    for i in range(NUM)[::-1]:
        for j in range(NUM):
            if [i, j] == head:
                print('H', end='')
            elif [i, j] in tails:
                print(tails.index([i, j])+1, end='')
            else:
                print('.', end='')
        print()
    print('\n')

def solve(file):
    dir_to_val = {'R': (0, 1), 'L': (0, -1), 'U': (1, 0), 'D': (-1, 0)}
    visited = [[False for _ in range(NUM)] for _ in range(NUM)]
    # if diff col and row move diagonally, otherwise orthogonally
    head = [NUM//2, NUM//2]
    tails = [[NUM//2, NUM//2] for _ in range(9)]
    for line in file:
        dir, num = line.strip().split(' ')
        dx, dy = dir_to_val[dir]
        num = int(num)
        for _ in range(num):
            head[0] += dx
            head[1] += dy
            tx, ty = move(head, tails[0])
            tails[0][0] += tx
            tails[0][1] += ty
            for i, tail in enumerate(tails[1:], 1):
                tx, ty = move(tails[i-1], tail)
                tails[i][0] += tx
                tails[i][1] += ty
            visited[tails[-1][0]][tails[-1][1]] = True
        # print_board(head, tails)
    return sum(sum(x) for x in visited)
    

def solution():
    with open('9/in.txt') as file:
        out = solve(file)
        print(out)
        if out is None:
            return
        with open('9/out.txt', 'w') as output:
            output.write(str(out))

if __name__ == '__main__':
    solution()