#!/usr/bin/env python
import aocd as ao
import numpy as np
import sys

def findSmallesFuelCost(data: np.ndarray, cost_func) -> int:
    min, max = np.min(data), np.max(data)
    smallesCount = sys.maxsize
    for i in range(min, max):
        count = 0
        for crab in data:
            if crab != i:
                count += cost_func(abs(i - crab))
        if count < smallesCount:
            smallesCount = count

    return smallesCount

def main():
    data = np.array(ao.data.split(","), dtype=np.int32)

    cost_func_stable = lambda n: n
    res = findSmallesFuelCost(data, cost_func_stable)

    print(f"Solution for part1 = {res}")
    #ao.submit(res)

    cost_func_grow = lambda n : int(n * (n + 1) / 2)
    res = findSmallesFuelCost(data, cost_func_grow)

    print(f"Solution for part2 = {res}")

if __name__ == '__main__':
    main()
