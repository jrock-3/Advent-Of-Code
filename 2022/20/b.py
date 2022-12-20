from collections import deque, defaultdict

year = 2022
num = 20
isB = True
test = False

filename = f'{year}/{num}/{"data" if not test else "test"}'
infile = f'{filename}.in'
outfile = f'{filename}_{"a" if not isB else "b"}.out'


class LL:
    def __init__(self, val):
        self.val = val
        self.prev = None
        self.next = None


def print_ll(root, length):
    out = []
    for _ in range(length):
        out.append(root.val)
        root = root.next
    print(', '.join(f'{x:>12}' for x in out))


def solver(file):
    nums = [int(x.strip())*811589153 for x in file]
    nodes = defaultdict(deque)
    root = None
    curr = None
    for num in nums:
        if root is None:
            root = LL(num)
            curr = root
        else:
            ll = LL(num)
            ll.prev = curr
            curr.next = ll
            curr = ll
        nodes[num].append(curr)
    root.prev = curr
    curr.next = root

    def sign(n):
        return 1 if n>0 else -1

    print([sign(x)*(abs(x)%(len(nums)-1)) for x in nums])
    # print_ll(root, len(nums))
    
    for _ in range(10):
        for num in nums:
            node = nodes[num].popleft()
            nodes[num].append(node)

            next_node, prev_node = node.next, node.prev
            if num == 0:
                continue
            if node == root:
                root = next_node if num > 0 else prev_node
            next_node.prev = prev_node
            prev_node.next = next_node
        
            curr = next_node if num > 0 else prev_node
            for _ in range(abs(num) % (len(nums)-1)):
                curr = curr.next if num > 0 else curr.prev

            if num > 0:
                lnode, rnode = curr.prev, curr
            else:
                lnode, rnode = curr, curr.next
            
            # insert between lnode and rnode
            lnode.next = node
            node.prev = lnode
            rnode.prev = node
            node.next = rnode

        # print_ll(root, len(nums))

    curr = root
    while curr.val != 0:
        curr = curr.next
    total = 0
    for _ in range(3):
        for _ in range(1000):
            curr = curr.next
        total += curr.val
    return total


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
