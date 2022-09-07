# 20_1b.py

nums = list()

with open('20_1.txt') as file:
    line = file.readline()

    while line != '':
        nums.append(int(line))
        line = file.readline()

for i, num1 in enumerate(nums):
    for j, num2 in enumerate(nums):
        for k, num3 in enumerate(nums):            
            if i!=j and j!=k and i!=k and num1 + num2 + num3 == 2020:
                print(num1 * num2 * num3)
                assert False
