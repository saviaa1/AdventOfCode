#!/usr/bin/env python
import aocd as ao
import numpy as np
from collections import Counter, defaultdict, deque
import math
import sys
import re

def pathHighest(lx, hx, ly, hy):
    #target area: x=175..227, y=-134..-79
    p1, p2 = 0, 0
    for DX in range(250):
        for DY in range(-140, 1000):
            x, y = 0, 0
            dx, dy = DX, DY
            max_y = 0
            ok = False
            for T in range(1000):
                x += dx
                y += dy
                max_y = max(max_y, y)
                if dx > 0:
                    dx -= 1
                elif dx < 0:
                    dx += 1
                dy -= 1
                if lx <= x <= hx and ly <= y <= hy:
                    ok = True
            if ok:
                p1 = max(max_y, p1)
                p2 += 1
    return p1, p2

def main():
    data = ao.data
    data_re = re.compile(r"x=(-*\d+)\.\.(-*\d+), y=(-*\d+)\.\.(-*\d+)")
    lx, hx, ly, hy = (int(coord) for coord in data_re.search(data).groups())

    print(data)

    p1, p2 = pathHighest(lx, hx, ly, hy)

    print(f"Solution for part1 = {p1}")
    #ao.submit(8911)

    p2 = p2

    print(f"Solution for part2 = {p2}")
    #ao.submit(4748)

if __name__ == '__main__':
    main()
