year = 2022
num = 22
isB = False
test = False

filename = f'{year}/{num}/{"data" if not test else "test"}'
infile = f'{filename}.in'
outfile = f'{filename}_{"a" if not isB else "b"}.out'


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
    dirchar = ['>', 'v', '<', '^']

    board = tiles.union(walls)
    maxX, maxY = max(x for x,_ in board), max(y for _,y in board)

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
    print_path(walls, tiles, maxX, maxY, path, posX, posY, start)
    print(posX, posY, didx)

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
