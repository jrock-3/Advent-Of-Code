input = '''
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
'''

def count_perms(springs: str, nums: list, changed: bool) -> int:
    if len(springs) == 0 or len(nums) == 0:
        print(int(len(nums) == 0))
        return len(nums) == 0
    if springs[0] == '.':
        if changed:
            if nums[0] != 0:
                print(0)
                return 0
            nums.pop(0)
        print('.',end='')
        return count_perms(springs[1:],nums, False)
    elif springs[0] == '#':
        if nums[0] == 0:
            return 0
        if nums[0] == 1:
            return count_perms(springs[1:],nums[1:], True)
        nums[0] -= 1
        print('#',end='')
        return count_perms(springs[1:],nums, True)

    # ?
    choose_dot = 0
    if not changed:
        print('.',end='')
        choose_dot = count_perms(springs[1:], nums, False)
    if nums[0] == 0:
        choose_pnd = 0
    elif nums[0] == 1:
        print('#',end='')
        choose_pnd = count_perms(springs[1:],nums[1:], True)
    else:
        nums[0] -= 1
        choose_pnd = count_perms(springs[1:],nums, True)
    return choose_dot + choose_pnd

res = 0
for line in input.strip().split('\n'):
    print(line)
    springs, nums = line.strip().split(' ')
    nums = list(map(int, nums.split(',')))
    res += count_perms(springs, nums, False)

print(res)
