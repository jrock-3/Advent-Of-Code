import itertools

year = 2022
num = 17
isB = False
test = False

filename = f'{year}/{num}/{"data" if not test else "test"}'
infile = f'{filename}.in'
outfile = f'{filename}_{"a" if not isB else "b"}.out'


pieces = {'-': [(2, 0), (3, 0), (4, 0), (5, 0)],
          '+': [(3, 0), (2, 1), (3, 1), (4, 1), (3, 2)],
          'j': [(2, 0), (3, 0), (4, 0), (4, 1), (4, 2)],
          'l': [(2, 0), (2, 1), (2, 2), (2, 3)],
          'o': [(2, 0), (2, 1), (3, 0), (3, 1)]}


def print_board(landed, rocks):
    rocks = {tuple(rock) for rock in rocks}
    for j in range(20)[::-1]:
        for i in range(7):
            ll = (i, j) in landed
            rr = (i, j) in rocks
            char = '#' if ll else ('@' if rr else '.')
            print(char, end='')
        print()
    print()


# 2 units from left, 3 units above highest rock
def solver(file):
    order = list('-+jlo')
    landed = set()
    moves = list(file)[0]
    # moves = (moves * (2022 // len(moves) + 1))[:2022]
    max_height = -1
    
    rocks = [[x, y+max_height+4] for x, y in pieces[order[0]]]
    order.append(order.pop(0))
    count = 0
    for move in itertools.cycle(moves):
        if move == '<':
            if all(x!=0 for x, _ in rocks) and all((x-1, y) not in landed for x, y in rocks):
                for i in range(len(rocks)):
                    rocks[i][0] -= 1
        else:
            if all(x!=6 for x, _ in rocks) and all((x+1, y) not in landed for x, y in rocks):
                for i in range(len(rocks)):
                    rocks[i][0] += 1
        
        if rocks[0][1] == 0 or any((x, y-1) in landed for x, y in rocks):
            for rock in rocks:
                landed.add(tuple(rock))
            max_height = max(max_height, rocks[-1][1])
            count += 1
            if count == 2022:
                break

            rocks = [[x, y+max_height+4] for x, y in pieces[order[0]]]
            order.append(order.pop(0))
            # if count < 11:
            #     print_board(landed, rocks)
        else:
            for i in range(len(rocks)):
                rocks[i][1] -= 1
    
    return max_height+1
        


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
