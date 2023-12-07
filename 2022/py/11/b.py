class Monkey:
    def __init__(self, items, operation, test, true_throw, false_throw):
        self.items = items
        self.operation = operation
        self.test = test
        self.tt = true_throw
        self.ft = false_throw
        self.count = 0
    
    def process(self, monkeys, n):
        for item in self.items:
            item = eval(self.operation, None, {'old': item})
            if item % self.test == 0:
                monkeys[self.tt].add_item(item % n)
            else:
                monkeys[self.ft].add_item(item % n)
        self.count += len(self.items)
        self.items = []
    
    def add_item(self, item):
        self.items.append(item)

def process(file):
    next(file)
    items = next(file).strip()[len('Starting items: '):].split(', ')
    operation = next(file).strip()[len('Operation: new = '):]
    test = int(next(file).strip().split(' ')[-1])
    true_throw = next(file).strip().split(' ')[-1]
    false_throw = next(file).strip().split(' ')[-1]
    return [int(item) for item in items], operation, test, int(true_throw), int(false_throw)


def solve(file):
    file = iter(file)
    monkeys = []
    while True:
        try:
            vals = process(file)
            monkeys.append(Monkey(*vals))
            next(file)
        except StopIteration:
            break
    
    # for monkey in monkeys:
        # print(monkey.items)
    n = 1
    for monkey in monkeys:
        n *= monkey.test
    for _ in range(10_000):
        for monkey in monkeys:
            monkey.process(monkeys, n)
        counts = [monkey.count for monkey in monkeys]
        # if x+1 in (1, 20, 1000):
        #     print(x+1)
        #     for i, count in enumerate(counts):
        #         print(f'Monkey {i} inspected items {count} times')
    counts = [monkey.count for monkey in monkeys]
    a, b = sorted(counts)[-2:]
    return a * b


def solution():
    with open('11/in.txt') as file:
        out = solve(file)
        print(out)
        if out is None:
            return
        with open('11/out.txt', 'w') as output:
            output.write(str(out))

if __name__ == '__main__':
    solution()