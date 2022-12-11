# 20_5b.py

seats = list()

with open('20_5.txt') as file:
    line = file.readline().strip('\n')

    while line != '':
        low = 0
        high = 128
        
        for char in line[:-3]:
            if char == 'F':
                high = (low + high) // 2
            elif char == 'B':
                low = (low + high) // 2

        row = low

        low = 0
        high = 8
        for char in line[-3:]:
            if char == 'L':
                high = (low + high) // 2
            elif char == 'R':
                low = (low + high) // 2

        col = low

        seats.append(row * 8 + col)

        line = file.readline().strip('\n')

##print(seats)

seats.sort()

potential = list()
for seat in seats:
    if seat + 1 not in seats:
        potential.append(seat + 1)
    if seat - 1 not in seats:
        potential.append(seat - 1)

print(potential[1:-1])
