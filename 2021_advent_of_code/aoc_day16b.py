# aoc_day16b.py

versions = list()

def handle_packet(header, sub=-1, limit=-1):
    total = list()
    original_len = len(header)
    while ((sub != -1 and sub > 0) or sub == -1) and ((limit != -1 and original_len-len(header)<limit) or limit == -1) and len(header) > 8:
        if sub != -1:
            sub -= 1

        version = int(header[:3], 2)
        versions.append(version)
        
        typeid = int(header[3:6], 2)

        header = header[6:]

##        print(header, 'hi')
        if typeid == 4: #4
            groups = list()
            while header[0] == '1':
                groups.append(header[1:5])
                header = header[5:]
            groups.append(header[1:5])
            header = header[5:]

            packet = ''
            for group in groups:
                packet += group
            packet = int(packet, 2)
##            print(f'{version=} {typeid=} {packet=}')
            total.append(packet)
        else: #not 4
            lenid = header[0]
            header = header[1:]
            if lenid == '0': #15
                _limit = int(header[:15], 2)
                header = header[15:]
##                print(f'{version=} {typeid=} {_limit=}')
                _total, header = handle_packet(header, limit=_limit)
                total.append([typeid, _total])
            elif lenid == '1': #11
                _sub = int(header[:11], 2)
                header = header[11:]
##                print(f'{version=} {typeid=} {_sub=}')
                _total, header = handle_packet(header, sub=_sub)
                total.append([typeid, _total])
    return total, header

def getval(v):
    if type(v) == int:
        return v
    return evaluate(v)

def evaluate(total):
    typeid, val = total

    if typeid == 0:
        def add(a,b):
            return a + b
        op = add
    elif typeid == 1:
        def prod(a,b):
            return a * b
        op = prod
    elif typeid == 2:
        op = min
    elif typeid == 3:
        op = max
    elif typeid == 5: #greater than
        return getval(val[0]) > getval(val[1])
    elif typeid == 6: #less than
        return getval(val[0]) < getval(val[1])
    elif typeid == 7: #equal to
        return getval(val[0]) == getval(val[1])

    output = getval(val.pop(0))
    for v in val:
        output = op(output, getval(v))
    return output


with open('aoc_day16.txt') as file:
    data = file.read().strip('\n')

header = str(bin(int(data, 16)))[2:]
for _ in range(len(data) * 4 - len(header)):
    header = '0' + header
##print(header)

total = handle_packet(header)[0]    
##print(total)

value = evaluate(total[0])
print(value)
