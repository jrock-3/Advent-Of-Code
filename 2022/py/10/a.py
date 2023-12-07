def solve(file):
    total = 0
    cycles = 1
    x = 1
    for line in file:
        add = False
        num = None
        line = line.strip()
        if line == 'noop':
            cycles += 1
        else:
            num = int(line.split(' ')[1])
            cycles += 1
            add = True
        
        if (cycles - 20) % 40 == 0:
            # print(line, x, cycles)
            total += x * cycles

        if add:
            x += num
            cycles += 1
            if (cycles - 20) % 40 == 0:
                # print(line, x, cycles)
                total += x * cycles
    return total



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