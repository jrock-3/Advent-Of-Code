# 20_8a.py

ops = list()

with open('20_8.txt') as file:
    line = file.readline().strip('\n')

    while line != '':
        instruction, value = line.split()
        value = int(value)

        ops.append((instruction, value))
        
        line = file.readline().strip('\n')

##print(ops)

acc = 0
visited = [False] * len(ops)

idx = 0
while 0 <= idx < len(ops) and not visited[idx]:
    visited[idx] = True
    instruction, value = ops[idx]
    if instruction == 'acc':
        acc += value
    elif instruction == 'jmp':
        idx += value
        continue
    idx += 1

print(acc)
