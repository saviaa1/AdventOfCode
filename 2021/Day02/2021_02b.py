#!/usr/bin/env python
import aocd as ao
import numpy as np

def main():
    data = np.genfromtxt(ao.lines, dtype=[("dir", "U25"), ("val", np.int32)])

    a = np.cumsum(                      #Cumulative sum of up and down values to form aimsum.
        np.where(                       #If dir is down return val, else goes to next where.
            data["dir"] == "down",
            data["val"],
            np.where(                   #If dir is up return -val, else 0
                data["dir"] == "up",
                -data["val"],
                0
            )
        ),
        dtype=np.int32
    )

    #Simply sums forward values.
    f = np.sum(data[data["dir"] == "forward"]["val"], dtype=np.int32)
    #Calculates depth based on cumulative sum of aim.
    d = np.sum(np.where(data["dir"] == "forward", data["val"] * a, 0), dtype=np.int32)

    print(f"Solution = {f*d}")
    #ao.submit(f*d)

if __name__ == '__main__':
    main()