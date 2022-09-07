# 20_2b.py

total = 0

with open('20_2.txt') as file:
    line = file.readline()

    while line != '':
        pos, letter, password = line.split()
        pos1, pos2 = pos.split('-')
        pos1 = int(pos1) - 1
        pos2 = int(pos2) - 1
        letter = letter[0]

        if (password[pos1] == letter) != (password[pos2] == letter):
            total += 1
        
        line = file.readline()

print(total)
