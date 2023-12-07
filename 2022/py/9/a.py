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

def print_board(head, tail):
    for i in range(NUM):
        for j in range(NUM):
            if [i, j] == head:
                print('H', end='')
            elif [i, j] == tail:
                print('T', end='')
            else:
                print('.', end='')
        print()
    print('\n')

def solve(file):
    dir_to_val = {'R': (0, 1), 'L': (0, -1), 'U': (1, 0), 'D': (-1, 0)}
    visited = [[False for _ in range(NUM)] for _ in range(NUM)]
    # if diff col and row move diagonally, otherwise orthogonally
    head = [0, 0]
    tail = [0, 0]
    for line in file:
        dir, num = line.strip().split(' ')
        dx, dy = dir_to_val[dir]
        num = int(num)
        for _ in range(num):
            head[0] += dx
            head[1] += dy
            tx, ty = move(head, tail)
            tail[0] += tx
            tail[1] += ty
            visited[tail[0]][tail[1]] = True
            # print_board(head, tail)
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