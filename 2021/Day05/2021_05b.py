#!/usr/bin/env python
import aocd as ao
import numpy as np

def markGrid(grid, data):
    for line in data:
        sx, sy= line[0,0], line[0,1]
        ex, ey = line[1,0], line[1,1]

        if sx == ex and sy == ey:
            grid[sx, sy] += 1
            continue

        dx = abs(ex - sx)
        dy = abs(ey - sy)
        minx = min(sx, ex)
        miny = min(sy, ey)

        if not (dx != 0 and dy != 0):
            for i in range(minx, minx + dx + 1):
                for j in range(miny, miny + dy + 1):
                    grid[i, j] += 1

        if dx == dy:
            stepx = 1 if sx < ex else -1
            stepy = 1 if sy < ey else -1
            for i, j in zip(range(sx, ex + stepx, stepx), range(sy, ey + stepy, stepy)):
                grid[i, j] += 1

    return grid

def main():
    data = ao.lines
    for i in range(0, len(data)):
        data[i] = data[i].split(" -> ")
        for j in range(0,2):
            data[i][j] = data[i][j].split(",")
    data = np.array(data, dtype=np.int32)
    
    grid = np.zeros((1000, 1000), dtype=np.int32)

    grid = markGrid(grid, data)

    grid = (grid[grid >= 2])
    sum = np.size(grid)

    print(f"Solution = {sum}")
    #ao.submit(sum)

if __name__ == '__main__':
    main()