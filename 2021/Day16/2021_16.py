#!/usr/bin/env python
import aocd as ao
import numpy as np
from collections import Counter, defaultdict, deque
import math

def parse(bits: str, i: int) -> tuple[int, int, int]:
    p1 = int(bits[i+0:i+3], 2)
    type_ = int(bits[i+3:i+6], 2)
    if type_ == 4:
        i += 6
        v = 0
        while True:
            v = v*16 + int(bits[i+1:i+5], 2)
            i += 5
            if bits[i-5] == "0":
                return p1, v, i
        assert False
    else:
        len_id = int(bits[i+6], 2)
        vs = []
        if len_id == 0:
            len_bits = int(bits[i+7:i+7+15], 2)
            start_i = i+7+15
            i = start_i
            while True:
                p1_t, v, next_i = parse(bits, i)
                p1 += p1_t
                vs.append(v)
                assert next_i > i
                assert next_i - start_i <= len_bits
                i = next_i
                if next_i - start_i == len_bits:
                    break
        else:
            n_packets = int(bits[i+7:i+7+11], 2)
            i += 7+11
            for t in range(n_packets):
                p1_t, v, next_i = parse(bits, i)
                p1 += p1_t
                vs.append(v)
                assert next_i > i
                i = next_i
        if type_ == 0:
            return p1, sum(vs), i
        elif type_ == 1:
            ans = math.prod(vs)
            return p1, ans, i
        elif type_ == 2:
            return p1, min(vs), i
        elif type_ == 3:
            return p1, max(vs), i
        elif type_ == 5:
            return p1, (1 if vs[0] > vs[1] else 0), i
        elif type_ == 6:
            return p1, (1 if vs[0] < vs[1] else 0), i
        elif type_ == 7:
            return p1, (1 if vs[0] == vs[1] else 0), i
        else:
            assert False

def main():
    data = ao.data.strip()
    binary = bin(int(data, 16))[2:]
    while len(binary) < 4*len(data):
        binary = '0'+binary
    assert len(binary)%4==0
    assert len(binary) == 4*len(data)
    
    p1, p2, _ = parse(binary, 0)
    res = p1

    print(f"Solution for part1 = {res}")
    #ao.submit(3587)

    res = p2

    print(f"Solution for part2 = {res}")
    #ao.submit(3906445077999)

if __name__ == '__main__':
    main()
