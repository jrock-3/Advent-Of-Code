def toValue(char):
    if char.isupper():
        return ord(char) + 26 - 64
    else:
        return ord(char) - 96

def solve(file):
    out = []
    vals = []
    for line in file:
        vals.append(set(char for char in line.rstrip('\n')))
        if len(vals) == 3:
            char = vals[0].intersection(vals[1]).intersection(vals[2]).pop()
            out.append(toValue(char))
            vals = []
    return sum(out)

with open('in.txt') as file:
    out = solve(file)
    print(out)
    # out = sum(out)
    with open('out.txt', 'w') as output:
        output.write(str(out))