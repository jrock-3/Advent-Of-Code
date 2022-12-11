# 20_2a.py

total = 0

with open('20_2.txt') as file:
    line = file.readline()

    while line != '':
        minmax, letter, password = line.split()
        low, high = minmax.split('-')
        low = int(low)
        high = int(high)
        letter = letter[0]

        count = 0
        for char in password:
            if char == letter:
                count += 1

        if low <= count <= high:
            total += 1
        
        line = file.readline()

print(total)
