#!/usr/bin/env python
import aocd as ao
import numpy as np

def checkValues(boards, r_num):
    i = 0
    while len(r_num) >= 0:
        num = r_num[0]
        r_num = r_num[1:]
        boards[boards == num] = -1
        if i >= 5:
            r, c, d = checkWins(boards)
            if (r != -1):
                return r, c, num
        i += 1
    return -1, -1, -1

def checkWins(boards: np.ndarray):
    # rows
    bingo_x, bingo_y = 0, 0
    for r in range(0, len(boards)):
        occurance = np.count_nonzero(boards[r] == -1)
        if occurance >= 5:
            bingo_x = r
            return r, 0, 0
    
    for r in range(0, len(boards), 5):
        for c in range(0, 5):
            counts = 0
            for i in range(0, 5):
                if boards[r+i][c] == -1:
                    counts += 1
            if counts >= 5:
                return r, c, 1
    return -1,-1,-1

def getWinningTable(boards, r, c):
    #x_cor = r - (r % 5)
    x_cor = 0
    sum_r = 0
    for i in range(0,5):
        row = boards[x_cor + i]
        row[row == -1] = 0
        sum_r += np.sum(row)
        print(f"{row} {sum_r}")
    return sum_r

    
def removeWinningTable(boards, r, c):
    x_cor = r - (r % 5)
    #print(f"del0 {len(boards)}")
    for i in range(0,5):
        boards = np.delete(boards, x_cor, 0)
    #print(f"del1 {len(boards)}")
    return boards


def main():
    data = ao.lines
    r_num = np.array(data[0].split(","), dtype=np.int32)

    data = list(filter(None, data[1:]))
    data = [string.split() for string in data]
    boards = np.array(data, dtype=np.int32)

    while len(boards) >= 5:
        r, c, num = checkValues(boards, r_num)
        if len(boards) == 5:
            break
        boards = removeWinningTable(boards, r, c)
        
    print(f"{r}, {c}, {num}")
    res = getWinningTable(boards, r, c) * num

    #print(boards)
    print(f"Solution = {res}")
    #ao.submit(res)

if __name__ == '__main__':
    main()