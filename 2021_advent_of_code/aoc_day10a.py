# aoc_day10a.py

values = {')': 3, ']': 57, '}': 1197, '>': 25137}
complement = {'(': ')', '[': ']', '{': '}', '<': '>'}
openers = list('([{<')

total = 0
queue = list()

with open('aoc_day10.txt') as file:
    line = file.readline().strip('\n')

    while line != '':
        for char in line:
            if char in openers:
                queue.append(char)
            if char in values:
                if char != complement[queue[-1]]:
                    total += values[char]
                    break
                else:
                    queue.pop()

        line = file.readline().strip('\n')

print(total)
