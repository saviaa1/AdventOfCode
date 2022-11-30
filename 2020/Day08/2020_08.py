#!/usr/bin/env python
import aocd as ao
from parse import search

def part1(data):
    visited_lines = set()
    acc: int = 0
    line: int = 0

    while True:
        if line in visited_lines:
            break
        
        visited_lines.add(line)

        inst, val = data[line]
        match inst:
            case "acc":
                acc += val
                line += 1
            case "jmp":
                line += val
            case "nop":
                line += 1
            case _:
                raise TypeError("Unknow type in match-case")
        
        if line == len(data):
            return acc, True
    return acc, False
    
def part2(data):
    for line in range(0, len(data)):
        copy_data = data.copy()
        inst, val = copy_data[line]
        match inst:
            case "jmp":
                copy_data[line] = ("nop", val)
            case "nop":
                copy_data[line] = ("jmp", val)
            case _:
                continue
        acc, valid = part1(copy_data)
        if valid:
            return acc
    return None

def main():
    data = ao.lines
    data = [search("{} {:d}", line).fixed for line in data]

    res, _ = part1(data)

    print(f"Solution part1 = {res}")
    #ao.submit(res)

    res = part2(data)
    print(f"Solution part2 = {res}")
    #ao.submit(res)

if __name__ == '__main__':
    main()