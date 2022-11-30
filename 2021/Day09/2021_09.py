#!/usr/bin/env python
import aocd as ao
import numpy as np
from collections import Counter

def countLowPoints(data: np.ndarray) -> int:
    risk_level: int = 0
    for i in range( len(data)):
        for j in range(len(data[0])):
            smaller = True
            for (di, dj) in [(i-1, j), (i+1, j), (i, j-1), (i, j+1)]:
                if di in range(len(data)) and dj in range(len(data[0])):
                    if data[i, j] >= data[di, dj]:
                        smaller = False
            if smaller:
                risk_level += data[i, j] + 1
    return risk_level

def countBasins(data: np.ndarray):
    def basin(i, j):
        down = None
        for (di, dj) in [(i-1, j), (i+1, j), (i, j-1), (i, j+1)]:
            if di in range(len(data)) and dj in range(len(data[0])):
                if data[i, j] > data[di, dj]:
                    down = (di, dj)
        if down is None:
            return (i, j)
        return basin(*down)

    basins = []
    for i in range(len(data)):
        for j in range(len(data[0])):
            if data[i,j] != 9:  
                basins.append(basin(i, j))
    
    ret = 1
    for _, common in Counter(basins).most_common(3):
        ret *= common  
    return ret


def main():
    data = ao.lines
    for i in range(0, len(data)):
        data[i] = [int(d) for d in data[i]]
    data = np.array(data, dtype=np.int8)

    #data2 = np.insert(data, 0, 9, axis=0)
    #data2 = np.insert(data2, 0, 9, axis=1)
    #data2 = np.insert(data2, len(data2), 9, axis=0)
    #data2 = np.insert(data2, len(data2)-1, 9, axis=1)

    res = countLowPoints(data)

    print(f"Solution for part1 = {res}")
    #ao.submit(res)

    res = countBasins(data)

    print(f"Solution for part2 = {res}")
    #ao.submit(res)

if __name__ == '__main__':
    main()
