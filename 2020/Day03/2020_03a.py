#!/usr/bin/env python
import aocd as ao
import numpy as np

def main():
    data = np.array(ao.lines)
    x, tree, length = 0, 0, np.char.str_len(data[0])
    for y in data:
        if y[x] == "#":
            tree += 1
        x = (x + 3) % length

    print(f"Solution = {tree}")
    #ao.submit(tree)

if __name__ == '__main__':
    main()
