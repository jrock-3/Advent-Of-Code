# aoc_day7b_optimal.py

with open('aoc_day7.txt') as file:
    crabs = [int(i) for i in file.readline().split(',')]

crabs.sort()

l = min(crabs)

total = sum([(crab - l) * ((crab - l) + 1) // 2 for crab in crabs])

idx = 0
new_total = total
sum1 = 0
sum2 = sum(crabs)
##print(new_total, change1, change2, sum1, sum2)
for i in range(l + 1, max(crabs) + 1):
    while idx < len(crabs) and crabs[idx] < i:
        sum1 += crabs[idx]
        sum2 -= crabs[idx]
        idx += 1
    
    new_total += idx * i - sum1
    new_total -= sum2 - (len(crabs) - idx) * (i - 1)

##    print(new_total, change1, change2, sum1, sum2)

    
    
    total = min(total, new_total)

print(total)

'''
crabs.sort()

l = min(crabs)

total = sum([(crab - l) * ((crab - l) + 1) // 2 for crab in crabs])

idx = 0
new_total = total
_sum1 = sum(crabs[:idx])
_sum2 = sum(crabs[idx:])
for i in range(l + 1, max(crabs) + 1):
    while idx < len(crabs) and crabs[idx] < i:
        idx += 1
        _sum1 += crabs[idx]
        _sum2 -= crabs[idx]
    
    new_total += _sum1
    new_total -= _sum2
    
    total = min(total, new_total)

print(total)
'''
'''
0,1,1,2,2,2, 4, 7, 14, 16
0 1 1 3 3 3 10 28 105 136
1 0 0 1 1 1  6 21 
i = 0
idx = 0

sum(crabs[:idx])

i = 1
idx = 3
new_total = total + 1 - 
'''
