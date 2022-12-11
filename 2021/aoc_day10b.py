# aoc_day10b.py

def do(lines):
    total = list()
    queue = ''
    idx = 0

    for line in lines:
        total.append(0)
        for char in line:
            if char in '([{<':
                queue += char
            elif char in ')]}>' and _complement[char] in queue:
                i = len(queue) - queue[::-1].index(_complement[char]) - 1
                queue = queue[0:i] + queue[i+1:]

        for char in queue[::-1]:
            total[idx] *= 5
            total[idx] += values[complement[char]]
        
        queue = ''
        idx += 1

    return total


values = {')': 1, ']': 2, '}': 3, '>': 4}
complement = {'(': ')', '[': ']', '{': '}', '<': '>'}
_complement = {v: k for k, v in complement.items()}
queue = ''

idx = 0
with open('aoc_day10.txt') as file:
    lines = file.read().split('\n')

i = 0
while i < len(lines):
    line = lines[i]
    for char in line:
        if char in complement:
            queue += char
        if char in values:
            if char != complement[queue[-1]]:
                del lines[i]
                i -= 1
                break
            else:
                queue = queue[:-1]
    i += 1

vals = do(lines)

vals.sort()

print(vals[len(vals) // 2])
