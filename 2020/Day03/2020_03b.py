#!/usr/bin/env python
import aocd as ao
import numpy as np

def treesInPath(rigth: int, down: int, data: np.ndarray) -> int:
    x, y, tree, length = 0, 0, 0, np.char.str_len(data[0])
    while y < data.size:
        if data[y][x] == "#":
            tree += 1
        x = (x + rigth) % length
        y += down
    return tree

def main():
    data = np.array(ao.lines)
    x1 = treesInPath(1,1, data)
    x2 = treesInPath(3,1, data)
    x3 = treesInPath(5,1, data)
    x4 = treesInPath(7,1, data)
    x5 = treesInPath(1,2, data)

    mul = x1*x2*x3*x4*x5
    print(f"Solution = {mul}")
    #ao.submit(mul)

if __name__ == '__main__':
    main()
