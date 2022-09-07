# aoc_day8a.py

count = 0

with open('aoc_day8.txt') as file:
    line = file.readline()

    while line != '':
        patterns, output = line.split('|')
        patterns = patterns.split()
        outputs = output.split()

        for output in outputs:
            if len(output) in (2, 3, 4, 7):
                count += 1
        
##        print(patterns, outputs)
        
        line = file.readline()

print(count)
