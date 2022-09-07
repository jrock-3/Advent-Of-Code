# aoc_day7b_optimal.py

with open('aoc_day7_test.txt') as file:
    crabs = [int(i) for i in file.readline().split(',')]

crabs.sort()

l = min(crabs)

total = sum([(crab - l) * ((crab - l) + 1) // 2 for crab in crabs])

idx = 0
new_total = total
sum1 = 0
sum2 = 0

print(new_total, sum1, sum2)

for i in range(l + 1, max(crabs) + 1):
    while idx < len(crabs) and crabs[idx] < i:
        idx += 1
        
        sum1 += crabs[idx]
        sum2 -= crabs[idx]
    
    new_total += sum1 + idx
    new_total -= sum2 - idx
    
    total = min(total, new_total)
    
    print(new_total, sum1, sum2)

print(total)

'''
    0, 1, 1, 2, 2, 2, 4, 7, 14, 16    t   c1   c2

0   0  1  1  3  3  3 10 28 105 136  290    0    0
1   1  0  0  1  1  1  6 21  91 120  242    1   49
2   3  1  1  0  0  0  3 15  78 105  206    4   40
3   6  3  3  1  1  1  1 10  66  91  183   10   33
4  10  6  6  3  3  3  0  6  55  78  170   16   29
5  15 10 10  6  6  6  1  3  45  66  168   23   25
6  21 15 15 10 10 10  3  1  36  55  176   30   22
7  28 21 21 15 15 15  6  0  28  45  194   37   19


    0, 1, 1, 2, 2, 2, 4, 7, 14, 16    t    d   dd  t-d  dtd

0   0  1  1  3  3  3 10 28 105 136  290    0  n/a  290  n/a
1   1  0  0  1  1  1  6 21  91 120  242    1    1  241   39
2   3  1  1  0  0  0  3 15  78 105  206    4    3  202   39
3   6  3  3  1  1  1  1 10  66  91  183   15   11  168   34
4  10  6  6  3  3  3  0  6  55  78  170   31   16  139   29
5  15 10 10  6  6  6  1  3  45  66  168   54   23  114   25
6  21 15 15 10 10 10  3  1  36  55  176   84   30   92   22
7  28 21 21 15 15 15  6  0  28  45  194  121   37   73   19

i = 1
idx = 3
sum1 += 1
sum2 -= 2 - 6 - 4 - 7 - 14 - 16
new_total += 1
new_total -= 
'''
