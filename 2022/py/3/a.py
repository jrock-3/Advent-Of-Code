def toValue(char):
    if char.isupper():
        return ord(char) + 26 - 64
    else:
        return ord(char) - 96

def solve(file):
    vals = []
    for line in file:
        sz = len(line) // 2
        a, b = line[:sz], line[sz:]
        a, b = set(char for char in a), set(char for char in b)
        char = a.intersection(b).pop()
        vals.append(toValue(char))
    return sum(vals)

with open('in.txt') as file:
    out = solve(file)
    print(out)
    # out = sum(out)
    with open('out.txt', 'w') as output:
        output.write(str(out))