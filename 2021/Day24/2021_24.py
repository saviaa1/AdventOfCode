#!/usr/bin/env python
import aocd as ao
import numpy as np
from collections import Counter, defaultdict, deque
import math
import sys
import ast
import re
import functools

sys.setrecursionlimit(10**7)

ops = []

@functools.lru_cache(maxsize=None)
def search(op_ind, w_val, x_val, y_val, z_val, p2):
    global ops
    if z_val > 10**8:
        return (False, "")
    if op_ind >= len(ops):
        return (z_val == 0, "")
    values = {"w": w_val, "x": x_val, "y": y_val, "z": z_val}

    def evaluate(var):
        return values[var] if var in values else int(var)
    
    op = ops[op_ind]

    if op[0] == "inp":
        if p2:
            for d in range(1, 10):
                values[op[1]] = d
                result = search(op_ind + 1, values["w"], values["x"], values["y"], values["z"], p2)

                if result[0]:
                    print(op_ind, w_val, x_val, y_val, z_val, str(d) + result[1])
                    return (True, str(d) + result[1])
        else:
            for d in range(9,0,-1):
                values[op[1]] = d
                result = search(op_ind + 1, values["w"], values["x"], values["y"], values["z"], p2)

                if result[0]:
                    print(op_ind, w_val, x_val, y_val, z_val, str(d) + result[1])
                    return (True, str(d) + result[1])
        return (False, 0)
    second = evaluate(op[2])

    if op[0] == "add":
        values[op[1]] += second
    elif op[0] == "mul":
        values[op[1]] *= second
    elif op[0] == "div":
        if second == 0:
            return (False, 0)
        values[op[1]] = int(values[op[1]] / second)
    elif op[0] == "mod":
        if values[op[1]] < 0 or second <= 0:
            return (False, 0)
        values[op[1]] %= second
    elif op[0] == "eql":
        values[op[1]] = 1 if values[op[1]] == second else 0
    else:
        assert False
    
    return search(op_ind + 1, values["w"], values["x"], values["y"], values["z"], p2)

def main():
    global ops
    lines = ao.lines
    ops = [line.split() for line in lines]
    print(ops)
    
    _, p1 = search(0,0,0,0,0, False)

    print(f"Solution for part1 = {p1}")
    #ao.submit() # 89959794919939 ok

    _, p2 = search(0,0,0,0,0, True)

    print(f"Solution for part2 = {p2}")
    #ao.submit() # 17115131916112 ok

if __name__ == '__main__':
    main()
