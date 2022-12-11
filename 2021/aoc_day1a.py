# aoc_day1a.py

count = 0

with open('aoc_day1.txt') as file:
    line = file.readline()
    next_line = file.readline()
    while next_line != '':
        if int(line) - int(next_line) < 0:
            count += 1
        line, next_line = next_line, file.readline()
    
print(count)
