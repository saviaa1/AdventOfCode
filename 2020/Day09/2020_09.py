#!/usr/bin/env python
import aocd as ao
import numpy as np

class Xmas:
    def __init__(self, preamble: list[int]):
        self.preamble = preamble

    def shift(self, input: int):
        self.preamble.append(input)
        self.preamble[1:]
    
    def is_valid(self, input: int):
        for i in self.preamble[:-1]:
            for j in self.preamble[1:]:
                if i + j == input: return True
        return False
    
    def part2(self, invalid: int) -> int:
        for i in range(0, len(self.preamble) - 1):
            for j in range(1, len(self.preamble) - 1):
                stack = np.array(self.preamble[i:j])
                if np.sum(stack) == invalid:
                    return np.min(stack) + np.max(stack)
        return None

def main():
    data = ao.numbers
    xmas = Xmas(data[:25])
    for i in data[25:]:
        if not xmas.is_valid(i):
            res = i
            break
        xmas.shift(i)

    print(f"Solution part1 = {res}")
    #ao.submit(res)
    
    xmas = Xmas(data)
    res = xmas.part2(res)
    print(f"Solution part2 = {res}")
    #ao.submit(res)

if __name__ == '__main__':
    main()