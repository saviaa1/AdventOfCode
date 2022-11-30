#!/usr/bin/env python
import aocd as ao
from itertools import permutations

# 0: abcefg     6
# 1: cf         2   !
# 2: acdeg      5
# 3: acdfg      5
# 4: bcdf       4   !
# 5: abdfg      5
# 6: abdefg     6
# 7: acf        3   !
# 8: abcdefg    7   !
# 9: abcdfg     6

def part1(data):
    res = sum(sum(len(digit) in {2,3,4,7} for digit in line[1]) for line in data)
    return res

def part2(data):
    digits = {
        0: 'abcefg',
        1: 'cf',
        2: 'acdeg',
        3: 'acdfg',
        4: 'bcdf',
        5: 'abdfg',
        6: 'abdefg',
        7: 'acf',
        8: 'abcdefg',
        9: 'abcdfg',
    }
    res = 0
    for line in data:
        #D = find_perm_slow(line[0], digits)
        D = find_perm_fast(line[0], digits)
        ret = ''
        for w in line[1]:
            w_perm = ''
            for c in w:
                w_perm += D[c]
            w_perm = ''.join(sorted(w_perm))
            d = [k for k, v in digits.items() if v==w_perm]
            assert len(d) == 1, len(d)
            ret += str(d[0])
        res += int(ret)
    return res

def  find_perm_slow(data, digits):
    for perm in permutations(list(range(8))):
        ok = True
        D = {}
        for i in range(8):
            D[chr(ord("a") + i)] = chr(ord("a") + perm[i])
        for digit in data:
            w_perm = ""
            for c in digit:
                w_perm += D[c]
            w_perm = "".join(sorted(w_perm))
            if w_perm not in digits.values():
                ok = False
                break
        if ok:
            return D

def find_perm_fast(data, _):
    D = {}
    cf = bd = eg = ""
    for w in data: # 1
        if len(w) == 2:
            cf = w
    assert len(cf) == 2, cf

    for w in data: # 6, 7
        if len(w) == 6 and (cf[0] in w) != (cf[1] in w):
            if cf[0] in w:
                D[cf[0]] = "f"
                D[cf[1]] = "c"
            else:
                D[cf[0]] = "c"
                D[cf[1]] = "f"
        elif len(w) == 3:
            for c in w:
                if c not in cf:
                    D[c] = "a"
        elif len(w) == 4:
            for c in w:
                if c not in cf:
                    bd += c
    assert len(D) == 3, D        
    assert len(bd) == 2, bd

    for w in data: # 0
        if len(w) == 6 and (bd[0] in w) != (bd[1] in w):
            if bd[0] in w:
                D[bd[0]] = "b"
                D[bd[1]] = "d"
            else:
                D[bd[0]] = "d"
                D[bd[1]] = "b"
    assert len(D) == 5, D

    for c in ["a", "b", "c", "d", "e", "f", "g"]:
        if c not in D:
            eg += c
    assert len(eg) == 2

    for w in data:                                          # 9
        if len(w) == 6 and (eg[0] in w) != (eg[1] in w):          # 9
            if eg[0] in w:
                D[eg[0]] = "g"
                D[eg[1]] = "e"
            else:
                D[eg[0]] = "e"
                D[eg[1]] = "g"
    assert len(D) == 7

    return D

def main():
    data = [[string.split() for string in line.split(" | ")] for line in ao.lines]

    res = part1(data)

    print(f"Solution for part1 = {res}")
    #ao.submit(res)

    res = part2(data)

    print(f"Solution for part2 = {res}")
    #ao.submit(res)

if __name__ == '__main__':
    main()
