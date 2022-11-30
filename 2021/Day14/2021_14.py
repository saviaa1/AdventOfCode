#!/usr/bin/env python
import aocd as ao
import numpy as np
from collections import Counter, defaultdict, deque

def insert(template: str, D: dict, loop) -> int:
    C1 = Counter()
    # Populate C1 from template
    for i in range(len(template)-1):
        C1[template[i] + template[i+1]] += 1
    
    # Loop through all insertions.
    for i in range(loop):
        C2 = Counter()
        for k in C1:
            C2[k[0] + D[k]] += C1[k]
            C2[D[k] + k[1]] += C1[k]
        C1 = C2

    # Count chars, because in C1 they are in pairs
    CF = Counter()
    for k in C1:
        CF[k[0]] += C1[k]
    CF[template[-1]] += 1
    
    # Return rsult of character count
    return max(CF.values()) - min(CF.values())

def main():
    data = ao.lines
    template = data[0].strip()
    D = {}
    for line in data[2:]:
        k, v = line.strip().split(" -> ")
        D[k] = v
    
    res = insert(template, D, 10)

    print(f"Solution for part1 = {res}")
    #ao.submit(3587)

    res = insert(template, D, 40)

    print(f"Solution for part2 = {res}")
    #ao.submit(3906445077999)

if __name__ == '__main__':
    main()
