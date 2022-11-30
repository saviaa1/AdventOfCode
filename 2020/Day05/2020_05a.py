#!/usr/bin/env python
import aocd as ao
import math

def geSeat(s: str):
    def recursive(hi: int, lo: int, i: int):
        if hi == lo:
            return hi
        if s[i] == "F" or s[i] == "L":
            return recursive(math.floor( hi - (hi-lo) / 2 ), lo, i + 1)
        else:
            return recursive(hi, hi - math.floor((hi - lo) / 2), i + 1)

    return recursive(127, 0, 0), recursive(7, 0, 7)

def main():
    data = ao.lines
    #data = list(["BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"])
    seat_ids = []
    for l in data:
        row, column = geSeat(l)
        seat_ids.append(row * 8 + column)

    print(f"Solution = {max(seat_ids)}")
    #ao.submit(max(seat_ids))

if __name__ == '__main__':
    main()
