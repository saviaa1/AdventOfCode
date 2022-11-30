#!/usr/bin/env python
import aocd as ao
from collections import Counter, defaultdict, deque

def solve(G: defaultdict[str, list], p2: bool) -> int:
    res = 0
    start = ("start", set(["start"]), None)
    Q = deque([start])
    while Q:
        pos, small, twice = Q.popleft()
        if pos == "end":
            res += 1
            continue
        for y in G[pos]:
            if y not in small:
                new_small = set(small)
                if y.islower():
                    new_small.add(y)
                Q.append((y, new_small, twice))
            elif y in small and twice is None and y not in ["start", "end"] and p2:
                Q.append((y, small, y))
    return res

def main():
    G = defaultdict(list)
    data = ao.lines
    for line in data:
        a, b = line.strip().split("-")
        G[a].append(b)
        G[b].append(a)

    res = solve(G, False)

    print(f"Solution for part1 = {res}")
    #ao.submit(res)

    res = solve(G, True)

    print(f"Solution for part2 = {res}")
    #ao.submit(res)

if __name__ == '__main__':
    main()
