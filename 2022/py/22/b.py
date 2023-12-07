year = 2022
num = 22
isB = True
test = False

filename = f'{year}/{num}/{"data" if not test else "test"}'
infile = f'{filename}.in'
outfile = f'{filename}_{"a" if not isB else "b"}.out'


'''
 _BA
 _C
ED
F

Ad, Cr
Ar, Dr
Cl, Eu
Bl, El
Bu, Fl
Dd, Fr
Au, Fd
'''


def print_path(walls, tiles, maxX, maxY, path, posX, posY, start):
    # print(path)
    for y in range(maxY):
        for x in range(maxX):
            coord = (x, y)
            c = ' '
            if coord in walls:
                c = '#'
            if coord in tiles:
                c = '.'
            if coord in path:
                c = path[coord]
            if coord == (start, 1):
                c = 'S'
            if coord == (posX, posY):
                c = 'E'
            print(c, end='')
        print()
    print()


def solver(file):
    tiles = set()
    walls = set()
    for y, line in enumerate(file, 1):
        if line[0].isnumeric():
            break
        for x, char in enumerate(line[:-1], 1):
            if char == '#':
                # handle wall
                walls.add((x, y))
            elif char == '.':
                # handle legal tile
                tiles.add((x, y))
    
    moves = []
    x = ''
    for char in line:
        if char.isnumeric():
            x += char
        else:
            if x != '':
                moves.append(int(x))
                x = ''
            moves.append(char)
    if x != '':
        moves.append(int(x))
    
    # order is right
    dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)]
    dir_to_letter = {(1, 0): 'r', (0, 1): 'd', (-1, 0): 'l', (0, -1): 'u'}
    letter_to_dir = {b: a for a, b in dir_to_letter.items()}
    dirchar = ['>', 'v', '<', '^']
    quadrants = {(2, 0): 'A', (1, 0): 'B', (1, 1): 'C',
                 (1, 2): 'D', (0, 2): 'E', (0, 3): 'F'}
    reverse_quadrants = {b: a for a, b in quadrants.items()}

    board = tiles.union(walls)
    maxX, maxY = max(x for x,_ in board), max(y for _,y in board)
    # print(maxX, maxY)

    G = [('Ad', 'Cr'), ('Ar', 'Dr'), ('Cl', 'Eu'), ('Bl', 'El'), ('Bu', 'Fl'), ('Dd', 'Fr'), ('Au', 'Fd')]

    connections = dict()
    for a, b in G:
        connections[a] = b
        connections[b] = a
    print(connections)

    didx = 0
    posX, posY = min(x for x,y in tiles if y == 1), 1
    start = posX
    path = dict()
    path[posX, posY] = dirchar[didx]
    for move in moves:
        # print(move)
        # print_path(tiles, walls, maxX, maxY, path)
        if type(move) is int:
            for _ in range(move):
                # move in direction by 1
                dx, dy = dirs[didx]
                newX, newY = posX+dx, posY+dy

                # handle wrapping
                # print(newX, newY)
                if (newX, newY) not in board:
                    quadrant = quadrants[(posX-1)//50, (posY-1)//50]
                    dir = dir_to_letter[dirs[didx]]
                    quadX, quadY = (posX-1)%50+1, (posY-1)%50+1
                    new_quadrant, new_dir = connections[quadrant+dir]
                    dx, dy = letter_to_dir[new_dir]
                    dx, dy = -1*dx, -1*dy
                    didx = dirs.index((dx, dy))
                    qx, qy = reverse_quadrants[new_quadrant]
                    
                    print(quadrant+dir, new_quadrant+new_dir, (qx, qy), (quadX, quadY), (posX, posY))

                    if quadY in (1, 50):
                        quadX, quadY = quadY, quadX
                    if dir == new_dir:
                        major_axis = 50 - major_axis + 1
                    newX, newY = qx*50+quadX, qy*50+quadY
                    if dy == 0:
                        newX, newY = newY, newX
                    print(posX, posY, newX, newY)
                    return

                    # print('wrap')
                    if dx == 0:
                        # use y
                        if dy > 0:
                            # get min
                            newY = min(y for x,y in board if x == posX)
                        else:
                            # get max
                            newY = max(y for x,y in board if x == posX)
                    else:
                        # use x
                        if dx > 0:
                            # get min
                            newX = min(x for x,y in board if y == posY)
                        else:
                            # get max
                            newX = max(x for x,y in board if y == posY)

                # handle wall
                if (newX, newY) in walls:
                    # print('hit wall')
                    break

                posX, posY = newX, newY

                # print(posX, posY)
                path[posX, posY] = dirchar[didx]
        else:
            didx = (didx + (1 if move == 'R' else -1)) % len(dirs)
    # print(path)
    # print_path(walls, tiles, maxX, maxY, path, posX, posY, start)
    # print(posX, posY, didx)

    return posY * 1000 + posX * 4 + didx


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
