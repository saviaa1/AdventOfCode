#!/usr/bin/env python
import aocd as ao
import numpy as np

def main():
    data = np.genfromtxt(ao.lines, dtype=["U6","U6","U25"])
    valid_pass = 0
    for a, b, passw in data:
        lo, hi = map(int, a.split("-"))
        char = b[0]
        if lo <= passw.count(char) <= hi:
            valid_pass += 1

    print(f"Solution = {valid_pass}")
    #ao.submit(valid_pass)

if __name__ == '__main__':
    main()
