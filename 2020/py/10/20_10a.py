# 20_10a.py

##def choose(adapters, idx):
##    i = 1
##    total = [adapters[idx]]
##    print(total[0], end=' ')
##    while i < len(adapters) - idx and adapters[idx + i] - adapters[idx] <= 3:
##        temp = choose(adapters, idx + i)
##        if temp[0] > total[-1]:
##            total.extend(temp)
##        i += 1
##    
##    return total

with open('20_10.txt') as file:
    adapters = [int(i) for i in file.read().split('\n')]

adapters.append(0)

adapters.sort()

##print(adapters)

ones = 0
threes = 1

for i in range(len(adapters) - 1):
    diff = adapters[i+1] - adapters[i]
##    print(diff, adapters[i], adapters[i+1])
    if diff == 3:
        threes += 1
    elif diff == 1:
        ones += 1

print(ones * threes)

##print('\n', choose(adapters, 0))
