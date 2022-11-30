#!/usr/bin/env python
import aocd as ao
import numpy as np

def main():
    data = ao.data.split("\n\n")
    data = [string.split("\n") for string in data]
    sum = 0
    for i in range(0, len(data)):
        common = set()
        for j in range(0, len(data[i])):
            data[i][j] = [char for char in data[i][j]]
            if j == 0:
                common = set(data[i][j])
            else:
                common.intersection_update(set(data[i][j]))
        sum += len(common)
    
    print(f"Solution = {sum}")
    #ao.submit(sum)

if __name__ == '__main__':
    main()