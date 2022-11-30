#!/usr/bin/env python
import aocd as ao
import numpy as np
from collections import Counter, defaultdict, deque
from copy import deepcopy

def solve(lines, p2: bool) -> int:
    A = [lines[2][3], lines[3][3]]
    B = [lines[2][5], lines[3][5]]
    C = [lines[2][7], lines[3][7]]
    D = [lines[2][9], lines[3][9]]
    if p2:
        A = [A[0], "D", "D", A[1]]
        B = [B[0], "C", "B", B[1]]
        C = [C[0], "B", "A", C[1]]
        D = [D[0], "A", "C", D[1]]
    TOP = []
    while len(TOP) < 11:
        TOP.append(".")
    start = ({"A": A, "B": B, "C": C, "D": D}, TOP)

    NEXT = {}
    DP = {}
    COST = {"A": 1, "B": 10, "C": 100, "D": 1000}

    def f(state):
        invariant(state, p2)
        bot, top = state
        key = to_key(state)
        if done(state):
            return 0
        if key in DP:
            return DP[key]
        for i,c in enumerate(top):
            if c in bot and can_move_to(c, bot[c]):
                if clear_path(c, i, top):
                    di = dest_idx(bot[c])
                    assert di is not None
                    dist = di + 1 + abs(bot_idx(c)-i)
                    cost = COST[c] * dist
                    new_top = list(top)
                    new_top[i] = "."
                    new_bot = deepcopy(bot)
                    new_bot[c][di] = c
                    NEXT[key] = (new_bot, new_top)
                    return cost + f((new_bot, new_top))
        ans = int(1e9)
        for k,col in bot.items():
            if not can_move_from(k, col):
                continue
            ki = top_idx(col)
            if ki is None:
                continue
            c = col[ki]
            for to_ in range(len(top)):
                if to_ in [2,4,6,8]:
                    continue
                if top[to_] != ".":
                    continue
                if clear_path(k, to_, top):
                    dist = ki + 1 + abs(to_ - bot_idx(k))
                    new_top = list(top)
                    assert new_top[to_] == "."
                    new_top[to_] = c
                    new_bot = deepcopy(bot)
                    assert new_bot[k][ki] == c
                    new_bot[k][ki] = "."
                    cost = COST[c] * dist + f((new_bot, new_top))
                    if cost < ans:
                        ans = cost
                        NEXT[key] = (new_bot, new_top)
                    ans = min(ans, COST[c] * dist + f((new_bot, new_top)))
        DP[key] = ans

        return ans
    ret = f(start)
    cur = start
    while not done(cur):
        show(cur, p2)
        cur = NEXT[to_key(cur)]
    show(cur, p2)
    return ret

def to_key(state):
    bot, top = state
    return (tuple((k, tuple(v)) for k,v in bot.items()), tuple(top))

def done(state):
    bot, top = state
    for k,v in bot.items():
        for vv in v:
            if vv != k:
                return False
    return True

def can_move_from(k, col):
    for c in col:
        if c != k and c != ".":
            return True
    return False

def can_move_to(k, col):
    for c in col:
        if c != k and c != ".":
            return False
    return True

def bot_idx(bot):
    return {"A": 2, "B": 4, "C": 6, "D": 8}[bot]

def top_idx(col):
    for i,c in enumerate(col):
        if c != ".":
            return i
    return None

def dest_idx(col):
    for i, c in reversed(list(enumerate(col))):
        if c== ".":
            return i
    return None

def between(a, bot, top):
    return (bot_idx(bot) < a < top) or (top < a < bot_idx(bot))
assert between(1, "A", 0)

def clear_path(bot, top_idx, top):
    for ti in range(len(top)):
        if between(ti, bot, top_idx) and top[ti] != ".":
            return False
    return True

def show(state, p2):
    bot, top = state
    assert len(top) == 11
    msg = []
    msg.append("")
    msg.append("#"*13)
    msg.append("#" + "".join(top) + "#")
    for i in range(4 if p2 else 2):
        msg.append("###" + "#".join([bot[k][i] for k in bot]) + "###")
    msg.append("#"*13)
    msg.append("")
    print("\n".join(msg))

def invariant(state, p2):
    bot, top = state
    C = Counter()
    for c in top:
        C[c] += 1
    for _, v in bot.items():
        for c in v:
            C[c] += 1
    rows = (4 if p2 else 2)
    assert C["A"] == rows
    assert C["B"] == rows
    assert C["C"] == rows
    assert C["D"] == rows
    assert C["."] == 11
    assert top[2] == "."
    assert top[4] == "."
    assert top[6] == "."
    assert top[8] == "."

def main():
    data = ao.lines
    
    p1 = solve(data, False)

    print(f"Solution for part1 = {p1}")
    ao.submit(p1, part=1)

    p2 = solve(data, True)

    print(f"Solution for part2 = {p2}")
    ao.submit(p2, part=2)

if __name__ == '__main__':
    main()
