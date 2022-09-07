# aoc_day1b.py

def update(nums):
    total = sum([int(i) for i in nums])
    _curr = total - int(nums[-1])
    _next = total - int(nums[0])

    return _curr, _next

count = 0

with open('aoc_day1.txt') as file:
    nums = [file.readline(), file.readline(), file.readline(), file.readline()]
    while nums[-1] != '':
        s1, s2 = update(nums)
        if s1 - s2 < 0:
            count += 1
        nums.pop(0)
        nums.append(file.readline())

print(count)
