# aoc_day7a_optimal.py

with open('aoc_day7_test.txt') as file:
    crabs = [int(i) for i in file.readline().split(',')]

crabs.sort()

x = crabs[len(crabs) // 2]

total = sum([abs(crab - x) for crab in crabs])

print(total)

'''
0,1,1,2,2,2,4,7,14,16
+6 -4
+5 -5
+6 -4
'''
