#!/usr/bin/env python
import aocd as ao
import numpy as np
from collections import Counter, defaultdict, deque
import math
import sys
import re

die = 0

def determ_roll():
    global die
    die += 1
    return die

def determinastic_game(p1, s1, p2, s2):
    global die
    r1 = determ_roll() + determ_roll() + determ_roll()
    p1 = (p1 + r1) % 10
    s1 += p1 + 1
    if s1 >= 1000:
        return s2 * die
    r2 = determ_roll() + determ_roll() + determ_roll()
    p2 = (p2 + r2) % 10
    s2 += p2 + 1
    if s2 >= 1000:
        return s1 * die
    return determinastic_game(p1, s1, p2, s2)

def dirac_game(p1, s1, p2, s2, DB):
    if s1 >= 21:
        return (1,0)
    if s2 >= 21:
        return (0,1)
    if (p1, s1, p2, s2) in DB:
        return DB[(p1, s1, p2, s2)]
    ans = (0, 0)
    for d1 in [1,2,3]:
        for d2 in [1,2,3]:
            for d3 in [1,2,3]:
                new_p = (p1+d1+d2+d3)%10
                new_s = s1 + new_p + 1
                x1, y1 = dirac_game(p2, s2, new_p, new_s, DB)
                ans = (ans[0]+y1, ans[1]+x1)
    DB[(p1, s1, p2, s2)] = ans
    return ans

def main():
    data = {}
    data_re = re.compile(r"Player (\d) starting position: (\d)")
    for line in ao.lines:
        player, start_pos = data_re.search(line).groups()
        data[int(player)] = int(start_pos)
    
    p1 = determinastic_game(data[1]-1, 0, data[2]-1, 0)

    print(f"Solution for part1 = {p1}")
    #ao.submit(p1, part=1)
    DB = {}
    p2 = max(dirac_game(data[1]-1, 0, data[2]-1, 0, DB))

    print(f"Solution for part2 = {p2}")
    #ao.submit(p2, part=2)

if __name__ == '__main__':
    main()
