#!/usr/bin/env python
import aocd as ao

def part1(data):
    points = {")": 3, "]": 57, "}": 1197, ">": 25137}
    char_pairs = {")": "(", "]": "[", "}": "{", ">": "<"}
    illegal_chars = []
    correct_data = []
    for line in data:
        stack = []
        not_corrupted = True
        for char in line:
            # check if valid, and if true pop stack
            if char in [")", "]", "}", ">"] and len(stack) > 0 and char_pairs[char] == stack[-1]:
                stack.pop()
            # if valid add stack
            elif char in ["(", "[", "{", "<"]:
                stack.append(char)
            # else add illegal and break
            else:
                illegal_chars.append(char)
                not_corrupted = False
                break
        if not_corrupted:
            correct_data.append(line)
    return sum([points[char] for char in illegal_chars]), correct_data

def part2(data):
    points = {"(": 1, "[": 2, "{": 3, "<": 4}
    char_pairs = {")": "(", "]": "[", "}": "{", ">": "<"}
    scores = []

    for line in data:
        stack = []
        for char in line:
            if char in [")", "]", "}", ">"] and len(stack) > 0 and char_pairs[char] == stack[-1]:
                stack.pop()
            elif char in ["(", "[", "{", "<"]:
                stack.append(char)
        score = 0
        stack.reverse()
        for char in stack:
            score = 5 * score + points[char]
        scores.append(score)

    scores.sort()
    return scores[(len(scores) - 1 ) // 2]

def main():
    data = ao.lines
    
    res, data = part1(data)

    print(f"Solution for part1 = {res}")
    #ao.submit(res)
    
    res = part2(data)

    print(f"Solution for part2 = {res}")
    #ao.submit(res)

if __name__ == '__main__':
    main()
