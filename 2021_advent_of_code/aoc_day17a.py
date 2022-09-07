# aoc_day17a.py

import math

with open('aoc_day17_test.txt') as file:
    data = file.read()

line = data[13:]
_x, _y = line.split(', ')
_x = _x[2:]
_x1, _x2 = _x.split('..')
_y = _y[2:]
_y1, _y2 = _y.split('..')

xrange = range(int(_x1), int(_x2)+1)
yrange = range(int(_y1), int(_y2)+1)

print(xrange, yrange)

xpos = 0
ypos = 0
xvel = 0
yvel = 0

heights = list()

'''
x = t*(dx_o - t*sign(dx))
y = t*(dy_o - t)

20 <= t*dx_o - t^2*sign(dx) <= 30
t = (sign(dx)*dx_o +- sqrt(dx_o^2 - 4A)) / 2A
test t for A in xrange

-10 <= t*dy_o - t^2 <= -5
t = (dy_o +- sqrt(dy_o^2 - 4B)/2B
test t for B in yrange
t*dy_o - t^2 = A
t^2 - dy_o*t + A
max y: t = dy_o/2
'''
def test(dx, dy):
    for x in xrange:
        for y in yrange:
            if dx*dx - 4*x < 0 or dy*dy - 4*y < 0:
                continue
            
            sign = 1 if dx > 0 else -1
            t1 = (sign*dx + math.sqrt(dx*dx - 4*x)) / 2*x
            t2 = (sign*dx - math.sqrt(dx*dx - 4*x)) / 2*x

            _t1 = (dy + math.sqrt(dy*dy - 4*y)) / 2*y
            _t2 = (dy - math.sqrt(dy*dy - 4*y)) / 2*y

            for t in (t1, t2):
                for _t in (_t1, _t2):
                    if t == _t and t > 0:
                        _y = t*t - dy*t + y
                        print(_y)

for dx in range(1,100):
    for dy in range(1,100):
        test(dx, dy)

##def step():
##    heights.append(ypos)
##    xpos += xvel
##    ypos += yvel
##
##    if xvel > 0:
##        xvel -= 1
##    elif xvel < 0:
##        xvel += 1
##
##    yvel -= 1
##
##def in_target(x, y, targetx, targety):
##    return x in targetx and y in targety
