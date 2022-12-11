# aoc_day8b.py

nums = {2: 1, 3: 7, 4: 4, 7: 8}

total = 0

with open('aoc_day8.txt') as file:
    line = file.readline()

    while line != '':
        translated = ''
        connections = [{}] * 10
        fives = list()
        sixes = list()
        
        patterns, outputs = line.split('|')
        
        patterns = [set(list(pattern)) for pattern in patterns.split()]
        
        outputs = [set(list(output)) for output in outputs.split()]

        for pattern in patterns:
            l = len(pattern)
            if l in [int(val) for val in nums.keys()]:
                connections[nums[l]] = pattern
            elif l == 5:
                fives.append(pattern)
            elif l == 6:
                sixes.append(pattern)

        # FIVES

        in_all = fives[0] & fives[1] & fives[2]
        _fives = [five - in_all for five in fives]
        
        unique5 = _fives[0] ^ _fives[1] ^ _fives[2]
        not_unique5 = _fives[0] - unique5 | _fives[1] - unique5 | _fives[2] - unique5
        
        for i, five1 in enumerate(_fives):
            test = set()
            for five2 in _fives:
                if five1 != five2:
                    test |= five1 & five2
            if len(test) == 2:
                connections[3] = fives[i]
                del fives[i]
                break


        # SIXES
        
        in_all = sixes[0] & sixes[1] & sixes[2]
        _sixes = [six - in_all for six in sixes]

        unique6 = _sixes[0] | _sixes[1] | _sixes[2]

        # SOLVING

        for i, five in enumerate(fives):
            if (unique6 & unique5).issubset(five):
                connections[2] = five
                del fives[i]
                connections[5] = fives[0]
                break

        special6 = unique6 - unique5 - not_unique5
        for i, six in enumerate(sixes):
            if not special6.issubset(six):
                connections[0] = six
                del sixes[i]
                break

        in_9 = connections[8] - connections[5] - connections[1]
        if not in_9.issubset(sixes[0]):
            connections[9] = sixes[0]
            connections[6] = sixes[1]
        else:
            connections[9] = sixes[1]
            connections[6] = sixes[0]

        # OUTPUT

        for output in outputs:
            try:
                translated += str(connections.index(output))
            except Exception:
                pass

        total += int(translated)
        
        line = file.readline()

print(total)
