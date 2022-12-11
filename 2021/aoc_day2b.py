# aoc_day2b.py

h_pos = d_pos = aim = 0

with open('aoc_day2.txt') as file:
    line = file.readline()
    while line != '':
        instruction, units = line.split()
        units = int(units)
        if instruction == 'forward':
            h_pos += units
            d_pos += aim * units
        elif instruction == 'down':
            aim += units
        else:
            aim -= units

        line = file.readline()

print(h_pos * d_pos)
