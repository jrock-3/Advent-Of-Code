year = 2022
num = 16
isB = False
test = True

filename = f'{year}/{num}/{"data" if not test else "test"}'
infile = f'{filename}.in'
outfile = f'{filename}_{"a" if not isB else "b"}.out'


def get_max(curr_valve, rates, costs, time):
    # base case no more time/valves to visit
    if len(rates) == 0 or time == 0:
        return 0
    
    max_val = 0

    # choose a valve to visit
    keys = set(rates.keys())
    for valve in keys:
        rate = rates[valve]
        # visit it
        t = time - costs[curr_valve, valve] - 1
        if t < 0:
            continue
        val = t * rate
        # print(curr_valve, valve, rate, time, t)

        # recurse
        del rates[valve]
        val += get_max(valve, rates, costs, t)
        rates[valve] = rate
        max_val = max(max_val, val)
    
    return max_val


def solver(file):
    neighbors = dict()
    rates = dict()
    for line in file:
        line = line.strip().split(' ')
        valve = line[1]
        rate = int(line[4][5:-1])
        other_valves = [x.strip(',') for x in line[9:]]
        
        neighbors[valve] = other_valves
        if rate != 0:
            rates[valve] = rate
    
    print(len(neighbors))

    costs = dict()
    # bfs over each valve w/ a rate to find cost
    # to go to each other valve w/ a rate (and 'AA')
    nodes = set(['AA'] + list(rates.keys()))
    for valve in nodes:
        lens = {valve: 0}
        queue = [valve]
        while len(queue) > 0:
            v = queue.pop(0)
            for n in neighbors[v]:
                if n not in lens:
                    lens[n] = lens[v] + 1
                    queue.append(n)
        for v in nodes.difference({valve}):
            costs[valve, v] = lens[v]
            costs[v, valve] = lens[v]
    
    return get_max('AA', rates, costs, 30)


def main():
    with open(infile) as file:
        s = solver(file)

    if s is None:
        return

    print(s)
    with open(outfile, 'w') as file:
        file.write(str(s))


if __name__ == '__main__':
    main()
