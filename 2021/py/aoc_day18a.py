# aoc_day18a_alsofailed.py

import json, copy

with open('aoc_day18_test.txt') as file:
    data = [json.loads(l) for l in file.read().split()]

class Node:
    def __init__(self):
        self.parent = None
        self.left = None
        self.right = None

    def __str__(self):
        output = '['
        if self.left != None:
            output += self.left.__str__() + ','
        if self.right != None:
            output += self.right.__str__() + ']'
        return output

    def __eq__(self, obj):
        return self.__str__() == obj.__str__()

    def inorderTraversal(self, root):
        res = list()
        res = self.inorderTraversal(root.left)
        res.append(root.data)
        res += self.inorderTraversal(root.right)
        return res

    def go_left(self):
        print(self)
        if type(self.left) == Node:
            print(self.left)
            return self.right.go_left()
        if type(self.left) == int:
            return self

    def get_left(self):
        print(self)
        if self.parent == None:
            return None
        if self.parent.left != self:
            if type(self.parent.left) == int:
                return self.parent
            return self.parent.left.go_right()
        else:
            return self.parent.get_left()

    def go_right(self):
        print(self)
        if type(self.right) == Node:
            return self.right.go_right()
        if type(self.right) == int:
            return self

    def get_right(self):
        print(self)
        if self.parent == None:
            return None
        if self.parent.right != self:
            if type(self.parent.right) == int:
                return self.parent
            print('hi')
            return self.parent.right.go_left()
        else:
            return self.parent.get_right()

def traverse(data):
    if type(data) == int:
        return data
    
    left, right = data
    node = Node()
    
    node.left = traverse(left)
    if type(node.left) == Node:
        node.left.parent = node

    node.right = traverse(right)
    if type(node.right) == Node:
        node.right.parent = node

    return node

root = traverse(data.pop(0))
##print(root)

def add(root, depth, a_root):
##    print(a_root, root, depth)
    if type(root.left) == Node:
        root.left = add(root.left, depth+1, a_root)
    elif type(root.left) == int and root.left >= 10:
        print(a_root, root, depth)
        
        node = Node()
        
        node.left = root.left // 2
        node.right = root.left - node.left
        
        node.parent = root
        
        root.left = node

        root.left = add(root.left, depth+1, a_root)
    if type(root.right) == Node:
        root.right = add(root.right, depth+1, a_root)
    elif type(root.right) == int and root.right >= 10:
        print(a_root, root, depth)
        
        node = Node()
        
        node.left = root.right // 2
        node.right = root.right - node.left
        
        node.parent = root
        
        root.right = node

        root.right = add(root.right, depth+1, a_root)

    if depth >= 4 and type(root.left) == int and type(root.right) == int:
        print(a_root, root, depth, root.parent)
##        print(root.left, root.right)
        l = root.get_left()
        r = root.get_right()
        print(f'{str(l)=} {str(r)=}')
        if r:
            if type(r.left) == int:
                r.left += root.right
            elif type(r.right) == int:
                r.right += root.right
        if l:
            if type(l.right) == int:
                l.right += root.left
            elif type(l.left) == int:
                l.left += root.left
        print('did it')

        return 0
    elif depth >= 4:
        if type(root.left) == Node:
            root.left = add(root.left, depth+1, a_root)
        if type(root.right) == Node:
            root.right = add(root.right, depth+1, a_root)
    
    return root

r = None
while root != r:
    r = copy.deepcopy(root)
    add(root, 0, root)

for line in data:
    _root = Node()
    
    _root.left = root
    _root.left.parent = _root
    
    _root.right = traverse(line)
    _root.right.parent = _root

    __root = None
    while _root != __root:
        __root = copy.deepcopy(_root)
        add(_root, 0, _root)
        print(_root)
    root = copy.deepcopy(_root)

print(root)
