#!/usr/bin/env python
import aocd as ao
import numpy as np
from collections import Counter, defaultdict, deque
import math
import sys
import ast
import re

def parse(data):
    X0 = set()
    Y0 = set()
    Z0 = set()
    C = []
    min_x = 0
    min_y = 0
    min_z = 0
    max_x = 0
    max_y = 0
    max_z = 0
    G = set()
    for r, line in enumerate(data.strip().split("\n")):
        words = line.split()
        cmd = words[0]
        x1, x2, y1, y2, z1, z2 = [int(x) for x in re.findall("-?\d+", words[1])]
        x1, x2 = min(x1, x2), max(x1, x2)
        y1, y2 = min(y1, y2), max(y1, y2)
        z1, z2 = min(z1, z2), max(z1, z2)

        X0.add(x1)
        X0.add(x2+1)
        Y0.add(y1)
        Y0.add(y2+1)
        Z0.add(z1)
        Z0.add(z2+1)

        min_x = min(x1, min_x)
        min_y = min(y1, min_y)
        min_z = min(z1, min_z)
        max_x = max(x2, max_x)
        max_y = max(y2, max_y)
        max_z = max(z2, max_z)

        C.append((x1,x2,y1,y2,z1,z2,cmd=="on"))

        X0.add(-50)
        X0.add(51)
        Y0.add(-50)
        Y0.add(51)
        Z0.add(-50)
        Z0.add(51)
        
        X,UX = expand(X0)
        Y,UY = expand(Y0)
        Z,UZ = expand(Z0)

    return (C, X, UX, Y, UY, Z, UZ)

def expand(A):
    B = set()
    for x in A:
        B.add(x)
    B = sorted(B)

    ret = {}
    U = {}
    len_sum = 0
    for i,x in enumerate(B):
        ret[x] = i
        if i+1 < len(B):
            len_ = B[i+1]-x if i+1<len(B) else None
            len_sum += len_
            U[i] = len_
    return (ret, U)


def solve(data, p1):
    C, X, UX, Y, UY, Z, UZ = parse(data)
    G = set()
    for t,(x1,x2,y1,y2,z1,z2,on) in enumerate(C):
        if p1:
            x1 = max(x1, -50)
            y1 = max(y1, -50)
            z1 = max(z1, -50)

            x2 = min(x2, 50)
            y2 = min(y2, 50)
            z2 = min(z2, 50)

        for x in range(X[x1], X[x2+1]):
            for y in range(Y[y1], Y[y2+1]):
                for z in range(Z[z1], Z[z2+1]):
                    if on:
                        G.add((x,y,z))
                    else:
                        G.discard((x,y,z))

    ans = 0
    for x,y,z in G:
        lx = UX[x]
        ly = UY[y]
        lz = UZ[z]
        ans += lx * ly * lz
    return ans

def main():
    data = ao.data
    
    p1 = solve(data, True)

    print(f"Solution for part1 = {p1}")
    #ao.submit() #576028

    p2 = solve(data, False)

    print(f"Solution for part2 = {p2}")
    #ao.submit()

if __name__ == '__main__':
    main()
