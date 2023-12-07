# 20_6b.py

answers = list()

with open('20_6.txt') as file:
    data = file.read().split('\n\n')

    for _ in range(len(data)):
        answers.append(set())

    for i, group in enumerate(data):
        for j, line in enumerate(group.split('\n')):
##            print(line)
##            print('hi')
            if j == 0:
                answers[i] |= set(line)
            else:
                answers[i] &= set(line)

##print(answers)

total = 0
for answer in answers:
    total += len(answer)

print(total)
