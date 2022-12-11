# 20_9a.py

with open('20_9.txt') as file:
    everything = [int(i) for i in file.read().split('\n')]

invalid = 675280050

for i in range(0, len(everything)):
    for j in range(2, len(everything)-i + 1):
        temp_list = everything[i:i+j]
        if sum(temp_list) == invalid:
            print(min(temp_list) + max(temp_list))
            assert False
