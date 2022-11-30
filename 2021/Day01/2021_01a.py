#!/usr/bin/env python
import aocd as ao
import sys

def main():
    i = 0
    prev = sys.maxsize
    for y in ao.numbers:
        if y > prev:
            i += 1
        prev = y
    
    print(f"Solution = {i}")
    #ao.submit(i)
    print(f"Sum = {sum([1,2])}")

if __name__ == '__main__':
    main()