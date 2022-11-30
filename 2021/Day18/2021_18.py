#!/usr/bin/env python
import aocd as ao
import numpy as np
from collections import Counter, defaultdict, deque
import math
import sys
import ast
import re

def add(n1, n2):
    return reduce([n1, n2])

def reduce(n):
    d1, n1 = explode(n)
    if d1:
        return reduce(n1)
    else:
        d2, n2 = split(n)
        if d2:
            return reduce(n2)
        else:
            return n2

def explode(n):
    ns = str(n)
    nums = re.findall('\d+', ns)
    parts = []
    i = 0
    while i < len(ns):
        if ns[i] == '[':
            parts.append('[')
            i += 1
        elif ns[i] == ',':
            parts.append(',')
            i += 1
        elif ns[i] == ']':
            parts.append(']')
            i += 1
        elif ns[i] == ' ':
            i += 1
        else:
            j = i
            while j < len(ns) and ns[j].isdigit():
                j += 1
            parts.append(int(ns[i:j]))
            i = j
    depth = 0
    for i,c in enumerate(parts):
        if c =="[":
            depth += 1
            if depth == 5:
                old_ns = ns
                left = parts[i+1]
                right = parts[i+3]
                left_i = None
                right_i = None
                for j in range(len(parts)):
                    if isinstance(parts[j], int) and j < i:
                        left_i = j
                    elif isinstance(parts[j], int) and j>i+3 and right_i is None:
                        right_i = j
                if right_i is not None:
                    parts[right_i] += right
                parts = parts[:i] + [0] + parts[i+5:]
                if left_i is not None:
                    parts[left_i] += left
                return True, ast.literal_eval("".join([str(x) for x in parts]))
        elif c == "]":
            depth -= 1
        else:
            pass
    return False, n

def split(n):
    if isinstance(n, list):
        d1, n1 = split(n[0])
        if d1:
            return True, [n1, n[1]]
        else:
            d2, n2 = split(n[1])
            return d2, [n1, n2]
    else:
        if n >= 10:
            return True, [n//2, (n+1)//2]
        else:
            return False, n

def magnitude(n):
    if isinstance(n, list):
        return 3 * magnitude(n[0]) + 2 * magnitude(n[1])
    return n

def main():
    data = []
    ans = None
    for line in ao.lines:
        line = line.strip()
        n = ast.literal_eval(line)
        data.append(n)
        if ans is None:
            ans = n
        else:
            ans = add(ans, n)
        p1 = magnitude(ans)

    print(f"Solution for part1 = {p1}")
    #ao.submit()

    p2 = None
    for x in data:
        for y in data:
            if x != y:
                score = magnitude(add(x,y))
                if p2 is None or score > p2:
                    p2 = score

    print(f"Solution for part2 = {p2}")
    #ao.submit()

if __name__ == '__main__':
    main()
