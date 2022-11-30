#!/usr/bin/env python
import aocd as ao
import numpy as np

def main():
    data = np.array(ao.data.split(","), dtype=np.int8)

    minus_one = np.vectorize(lambda v: v - 1)
    
    assert np.sum(data  == 0) == 0, "Presumed 0 days until birth is not possibel at beginning."

    for i in range(1, 81):
        print(f"Day {i}")

        count = np.sum(data == 0)
        data[data == 0] = 7
        data = minus_one(data)
        a = np.full(count, fill_value=8, dtype=np.int8)
        data = np.append(data, a)

    sum = np.size(data)
    print(f"Solution = {sum}")
    #ao.submit(sum)

if __name__ == '__main__':
    main()