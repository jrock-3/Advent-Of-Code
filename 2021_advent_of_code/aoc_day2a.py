# aoc_day2a.py

h_pos = d_pos = 0

with open('aoc_day2.txt') as file:
    line = file.readline()
    while line != '':
        instruction, units = line.split()
        units = int(units)
        if instruction == 'forward':
            h_pos += units
        elif instruction == 'down':
            d_pos += units
        else:
            d_pos -= units

        line = file.readline()

print(h_pos * d_pos)
