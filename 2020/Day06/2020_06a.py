#!/usr/bin/env python
import aocd as ao
import numpy as np

def main():
    data = ao.data.split("\n\n")
    data = [string.split("\n") for string in data]
    data = np.array(["".join(string) for string in data])

    sum = np.sum(list(map(lambda x: len(set("".join(x))), data)))

    print(f"Solution = {sum}")
    #ao.submit(sum)

if __name__ == '__main__':
    main()
