#!/usr/bin/env python
import aocd as ao
import numpy as np
from collections import Counter, defaultdict

def flash(data, i, j) -> int:
    ret = 1
    data[i, j] = -1
    for x in [-1, 0 , 1]:
        for y in [-1, 0, 1]:
            if 0 <= i + x < len(data[0]) and 0 <= j + y < len(data) and data[i+x, j+y] != -1:
                data[i+x, j+y] += 1
                if data[i+x, j+y] > 9:
                    ret += flash(data, i+x, j+y)
    return ret

def solution(data: np.ndarray) -> int:
    res = 0
    t = 0
    while True:
        t += 1
        data += 1
        for i in range(len(data[0])):
            for j in range(len(data)):
                if data[i, j] > 9:
                        numOfFlashes = flash(data, i, j)
                        if t <= 100:
                            res += numOfFlashes
        if np.count_nonzero(data > -1) == 0:
            break
        data[data == -1] = 0
    return res, t

def main():
    data = np.array([[d for d in line] for line in ao.lines], dtype=np.int8)

    res, t = solution(data)

    print(f"Solution for part1 = {res}")
    #ao.submit(res)

    res = t

    print(f"Solution for part2 = {res}")
    #ao.submit(res)

if __name__ == '__main__':
    main()
