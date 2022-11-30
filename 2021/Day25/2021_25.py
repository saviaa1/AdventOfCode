#!/usr/bin/env python
import aocd as ao
import numpy as np
from collections import Counter, defaultdict, deque
import math
import sys
import ast
import re
import copy

def solve(data) -> int:
    R = len(data)
    C = len(data[0])
    t = 0
    while True:
        t += 1
        moved = False
        data2 = copy.deepcopy(data)
        for r in range(R):
            for c in range(C):
                if data[r][c] == ">" and data[r][(c+1)%C] == ".":
                    data2[r][c] = "."
                    data2[r][(c+1)%C] = ">"
                    moved = True
        data3 = copy.deepcopy(data2)
        for r in range(R):
            for c in range(C):
                if data2[r][c] == "v" and data2[(r+1)%R][c] == ".":
                    data3[r][c] = "."
                    data3[(r+1)%R][c] = "v"
                    moved = True
        data = data3
        if not moved:
            return t

def printArr(data):
    for r in range(len(data)):
        line = ""
        for c in range(len(data[0])):
            line += data[r][c]
        print(line)

def main():
    data = ao.lines
    data = [[data[r][c] for c in range(len(data[0]))] for r in range(len(data))]

    #printArr(data)

    p1 = solve(data)

    print(f"Solution for part1 = {p1}")
    #ao.submit() #515

    #p2 = 0

    #print(f"Solution for part2 = {p2}")
    #ao.submit()

if __name__ == '__main__':
    main()
