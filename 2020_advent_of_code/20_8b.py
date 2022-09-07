# 20_8b.py

def run(ops):
    acc = 0
    visited = [False] * len(ops)

    idx = 0
    while 0 <= idx < len(ops):
        if visited[idx]:
            return False, acc
        visited[idx] = True
        instruction, value = ops[idx]
        if instruction == 'acc':
            acc += value
        elif instruction == 'jmp':
            idx += value
            continue
        idx += 1

    return True, acc

ops = list()

with open('20_8.txt') as file:
    line = file.readline().strip('\n')

    while line != '':
        instruction, value = line.split()
        value = int(value)

        ops.append([instruction, value])
        
        line = file.readline().strip('\n')

for i, op in enumerate(ops):
    if op[0] == 'jmp':
        ops[i][0] = 'nop'
        if run(ops)[0]:
            break
        ops[i][0] = 'jmp'
    elif op[0] == 'nop':
        ops[i][0] = 'jmp'
        if run(ops)[0]:
            break
        ops[i][0] = 'nop'

print(run(ops)[1])
