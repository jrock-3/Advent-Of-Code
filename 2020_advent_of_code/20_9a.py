# 20_9a.py

PREAMBLE = 25

preamble = list()

with open('20_9.txt') as file:
    for i in range(PREAMBLE):
        line = file.readline().strip('\n')

        preamble.append(int(line))

    while True:
        line = file.readline().strip('\n')

        for i, num in enumerate(preamble):
            other = int(line) - num
            if other in preamble and preamble.index(other) != i:
                break
        else:
            print(line)
            break

        preamble.pop(0)
        preamble.append(int(line))
