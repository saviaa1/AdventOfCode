#!/usr/bin/env python
import aocd as ao
import numpy as np
from collections import Counter, defaultdict, deque

def fold(C: dict, F: list, p1: bool) -> int:
    for f, c in F:
        C2 = {}
        if f == "x":
            for x, y in C:
                if x < c:
                    C2[(x, y)] = True
                else:
                    C2[((c - (x - c)), y)] = True
        elif f == "y":
            for x, y in C:
                if y < c:
                    C2[(x, y)] = True
                else:
                    C2[(x, c - (y - c))] = True
        else:
            assert False
        C = C2
        if p1:
            return len(C)
    return C

def stringify(C: dict) -> str:
    ans = ""
    for y in range(max([y for x, y in C.keys()]) + 1):
        for x in range(max(x for x, y in C.keys()) + 1):
            ans += "x" if (x, y) in C else " "
        ans += "\n"
    return ans

def main():
    C = {}
    F = []
    for line in ao.lines:
        if line.startswith("fold"):
            axsis, where = (line.split()[-1]).split("=")
            where = int(where)
            F.append((axsis, where))
        elif line:
            x, y = [int(n) for n in line.split(",")]
            C[(x, y)] = True

    res = fold(C, F, True)

    print(f"Solution for part1 = {res}")
    #ao.submit(res)

    res = stringify(fold(C, F, False))

    print(f"Solution for part2 = \n{res}")
    #ao.submit("PERCGJPB")

if __name__ == '__main__':
    main()
