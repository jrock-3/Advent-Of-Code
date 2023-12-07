def draw(x, cycles):
    isDrawn = x - 1 <= (cycles - 1) % 40 <= x + 1
    char = '#' if isDrawn else '.'
    # print(f'during cycle {cycles:>3}: CRT draws {char}')
    print(char, end='')
    if cycles % 40 == 0:
        print()


def solve(file):
    cycles = 0
    x = 1
    for line in file:
        line = line.strip()
        if line == 'noop':
            cycles += 1
            # print(f'start cycle  {cycles:>3}: begin executing noop')
            draw(x, cycles)
            # print(f'end of cycle {cycles:>3}: finish executing noop')
        else:
            cycles += 1
            # print(f'start cycle  {cycles:>3}: begin executing addx')
            draw(x, cycles)
            cycles += 1
            draw(x, cycles)
            x += int(line.split(' ')[1])
            # print(f'end of cycle {cycles:>3}: finish executing addx (X is now {x})')


def solution():
    with open('10/in.txt') as file:
        out = solve(file)
        print(out)
        if out is None:
            return
        with open('9/out.txt', 'w') as output:
            output.write(str(out))

if __name__ == '__main__':
    solution()