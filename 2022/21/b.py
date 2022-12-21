import operator

year = 2022
num = 21
isB = True
test = False

filename = f'{year}/{num}/{"data" if not test else "test"}'
infile = f'{filename}.in'
outfile = f'{filename}_{"a" if not isB else "b"}.out'


def get_val(vars, var):
    vals = vars[var]
    if type(vals) is int:
        return vals
    
    if vals == 'humn':
        return vals
    
    v1, op, v2 = vals
    v1, v2 = get_val(vars, v1), get_val(vars, v2)
    if v1 == 'humn':
        vars[var] = (v1, op, v2) # res
        return vars[var]
    
    if v1 == 'humn':
        vars[var] = (v1, op, v2) # res
        return vars[var]
    
    if type(v1) is tuple or type(v2) is tuple:
        vars[var] = (v1, op, v2)
    else:
        vars[var] = op(v1, v2)
    return vars[var]


def inverse(vars, root, val):
    inv = {operator.add: operator.sub, operator.sub: operator.add, operator.mul: operator.floordiv, operator.floordiv: operator.mul}
    inv_rev = {operator.add: operator.sub, operator.sub: operator.sub, operator.mul: operator.floordiv, operator.floordiv: operator.floordiv}
    # print(root, val)
    if root == 'humn':
        return val
    v1, op, v2 = root
    if op in (operator.add, operator.mul):
        if not (type(v1) is tuple or type(v1) is str):
            v1, v2 = v2, v1
        return inverse(vars, v1, inv[op](val, v2))

    if type(v1) is tuple or type(v1) is str:
        return inverse(vars, v1, inv[op](val, v2))
    else:
        return inverse(vars, v2, inv_rev[op](v1, val))


def solver(file):
    operators = {'+': operator.add, '-': operator.sub, '/': operator.floordiv, '*': operator.mul}
    vars = dict()
    for line in file:
        line = line.strip().split(' ')
        var = line[0][:-1]
        if var == 'humn':
            vars[var] = var
            continue

        if len(line) == 4:
            vars[var] = [line[1], operators[line[2]], line[3]]
        else:
            vars[var] = int(line[-1])

    get_val(vars, 'root')
    # print(vars['root'])

    v1, _, v2 = vars['root']
    v1 = inverse(vars, v1, v2)
    return v1


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
