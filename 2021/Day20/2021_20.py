#!/usr/bin/env python
import aocd as ao
import numpy as np
from collections import Counter, defaultdict, deque
import itertools
import math
import sys
import ast
import re

def solve(G, rule, on):
    G2 = set()
    rlo = min([r for r,c in G])
    rhi = max([r for r,c in G])
    clo = min([c for r,c in G])
    chi = max([c for r,c in G])
    for r in range(rlo-5, rhi+10):
        for c in range(clo-5, chi+10):
            rc_str = 0
            bit = 8
            for dr in [-1, 0, 1]:
                for dc in [-1, 0, 1]:
                    if ((r+dr,c+dc) in G) == on:
                        rc_str += 2**bit
                    bit -= 1
            assert 0 <= rc_str < 512
            if (rule[rc_str] == "#") != on:
                G2.add((r,c))
    return G2

def main():
    #data = open("ex.txt").read().strip()
    data = ao.data.strip()
    rule, image = data.split('\n\n')
    rule = rule.strip()
    assert len(rule) == 512
    G = set()
    for r, line in enumerate(image.strip().split("\n")):
        for c, x in enumerate(line.strip()):
            if x=='#':
                G.add((r,c))

    for t in range(50):
        if t == 2:
            p1 = len(G)
        G = solve(G, rule, t%2==0)
    p2 = len(G)

    print(f"Solution for part1 = {p1}")
    #ao.submit(p1, part=1)

    print(f"Solution for part2 = {p2}")
    #ao.submit(p2, part=2)

if __name__ == '__main__':
    main()
