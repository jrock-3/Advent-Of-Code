# 20_1a.py

nums = list()

with open('20_1.txt') as file:
    line = file.readline()

    while line != '':
        nums.append(int(line))
        line = file.readline()

for i, num1 in enumerate(nums):
    for j, num2 in enumerate(nums):
        if i != j and num1 + num2 == 2020:
            print(num1 * num2)
            assert False
