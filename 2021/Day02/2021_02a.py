#!/usr/bin/env python
import aocd as ao
import numpy as np

def main():
    data = np.genfromtxt(ao.lines, dtype=[("dir", "U25"), ("val", np.int32)])

    f = np.sum(data[data["dir"] == "forward"]["val"], dtype=np.int32)
    d = np.sum(data[data["dir"] == "down"]["val"], dtype=np.int32) - np.sum(data[data["dir"] == "up"]["val"], dtype=np.int32)

    print(f"Solution = {f*d}")
    #ao.submit(f*d)

if __name__ == '__main__':
    main()