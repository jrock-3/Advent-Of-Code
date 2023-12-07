# aoc_day16a.py

'''

ver 3
2 subpackets
00000000000000000101100001000101010110001011001000100000000010000100011000111000110100


'''

versions = list()

def handle_packet(header, sub=-1):
    while ((sub != -1 and sub > 0) or sub == -1) and len(header) > 8:
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
            print(f'{version=} {typeid=} {packet=}')
        else: #not 4
            lenid = header[0]
            header = header[1:]
            if lenid == '0': #15
                sublen = int(header[:15], 2)
                header = header[15:]#15+sublen]
                print(f'{version=} {typeid=} {sublen=}')
            elif lenid == '1': #11
                _sub = int(header[:11], 2)
                header = header[11:]
                print(f'{version=} {typeid=} {sub=}')
                header = handle_packet(header, _sub)
    return header


with open('aoc_day16.txt') as file:
    data = file.read().strip('\n')

header = str(bin(int(data, 16)))[2:]
for _ in range(len(data) * 4 - len(header)):
    header = '0' + header
##print(header)

handle_packet(header)
    
print(sum(versions))
