#!/usr/bin/env python
import aocd as ao
import numpy as np
import sys

def main():
    data = np.array(ao.numbers, dtype=int)
    data = np.convolve(data, np.ones(3, dtype=int), "valid")

    i = 0
    prev = sys.maxsize
    for x in data:
        if x > prev:
            i += 1
        prev = x
    print(f"Solution = {i}")
    #ao.submit(i)

if __name__ == '__main__':
    main()