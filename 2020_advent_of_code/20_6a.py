# 20_6a.py

answers = list()

with open('20_6.txt') as file:
    data = file.read().split('\n\n')

    for _ in range(len(data)):
        answers.append(set())

    for i, group in enumerate(data):
        for line in group:
            for char in line.strip('\n'):
                answers[i] |= set(char)

##print(answers)

total = 0
for answer in answers:
    total += len(answer)

print(total)
