# aoc_day6a.py

'''
make an array to keep track

store first element
shift everything once to the left, wrap around
add stored number to sixth element

idx:
  012345678
values:
0 011210000
1 112100000
2 121000101
3 210001111
4 100011312
5 000113221

'''

times = [0] * 9

with open('aoc_day6.txt') as file:
    line = file.readline()

    for time in line.split(','):
        times[int(time)] += 1

for i in range(256):
    rollover = times.pop(0)
    times.append(rollover)
    times[6] += rollover

print(sum(times))
