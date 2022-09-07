# 20_3a.py

count = 0

with open('20_3.txt') as file:
    file.readline()

    line = file.readline().strip('\n')

    x = 1
    while line != '':
        if line[(3 * x) % len(line)] == '#':
            count += 1
        
        x += 1
        line = file.readline().strip('\n')

print(count)
