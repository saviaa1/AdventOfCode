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

    #Filter out first and last row
    a = list(filter(lambda x: x > 0*8+4 or x < 127*8+0, seat_ids))

    list_missing_seats = [seat for seat in range(min(seat_ids), max(seat_ids)) if seat not in seat_ids]
    print(f"Solution = {list_missing_seats[0]}")
    #ao.submit(list_missing_seats[0])

if __name__ == '__main__':
    main()
