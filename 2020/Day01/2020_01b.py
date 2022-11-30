#!/usr/bin/env python
import numpy as np
import aocd as ao
import sys
import time

def main():
    data = np.array(ao.numbers, dtype=np.int32)
    start = time.perf_counter_ns()
    data = np.sort(data)
    for i in data:
        for j in data:
            for k in data:
                if i + j + k == 2020:
                    if i + j + k != 2020:
                        break
                    print(f"Solution = {i*j*k} ({i}, {j}, {k})")
                    end = time.perf_counter_ns()
                    #ao.submit(i*j*k)
                    print(f"time {(end-start)/1e6} ms")
                    sys.exit(0)
            if i + j + data[0] > 2020:
                break

if __name__ == '__main__':
    main()