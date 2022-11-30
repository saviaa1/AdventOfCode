#!/usr/bin/env python
import aocd as ao
import numpy as np
import sys

def main():
    data = np.array(ao.numbers, dtype=np.int32)
    data = np.sort(data)
    for i in data:
        for j in data:
            if i + j >= 2020:
                if i + j != 2020:
                    break
                print(f"Solution = {i*j} ({i}, {j})")
                #ao.submit(i*j)
                sys.exit(0)

if __name__ == '__main__':
    main()