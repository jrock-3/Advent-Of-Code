import operator

year = 2022
num = 21
isB = False
test = False

filename = f'{year}/{num}/{"data" if not test else "test"}'
infile = f'{filename}.in'
outfile = f'{filename}_{"a" if not isB else "b"}.out'


def get_val(vars, var):
    vals = vars[var]
    if type(vals) is int:
        return vals
    v1, op, v2 = vals
    res = op(get_val(vars, v1), get_val(vars, v2))
    vars[res] = res
    return res


def solver(file):
    operators = {'+': operator.add, '-': operator.sub, '/': operator.floordiv, '*': operator.mul}
    vars = dict()
    for line in file:
        line = line.strip().split(' ')
        var = line[0][:-1]
        if len(line) == 4:
            vars[var] = [line[1], operators[line[2]], line[3]]
        else:
            vars[var] = int(line[-1])

    return get_val(vars, 'root')


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
