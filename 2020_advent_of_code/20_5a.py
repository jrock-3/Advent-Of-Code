# 20_5a.py

highest = 0

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

        highest = max(row * 8 + col, highest)

##        print(row * 8 + col)

        line = file.readline().strip('\n')

print(highest)
