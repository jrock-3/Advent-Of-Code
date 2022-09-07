# 20_12b.py

w_x = 10
w_y = 1
s_x = 0
s_y = 0

with open('20_12.txt') as file:
    data = file.read().split('\n')

for line in data:
    i = line[0]
    v = int(line[1:])

    if i == 'N':
        w_y += v
    elif i == 'S':
        w_y -= v
    elif i == 'E':
        w_x += v
    elif i == 'W':
        w_x -= v
    elif i in 'LR':
        for _ in range(v // 90):
            if i == 'R':
                w_x, w_y = w_y, -w_x
            else:
                w_x, w_y = -w_y, w_x
    elif i == 'F':
        s_x += v * w_x
        s_y += v * w_y

print(abs(s_x) + abs(s_y))
