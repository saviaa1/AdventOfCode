#!/usr/bin/env python
import aocd as ao

def convListTodict(list: list) -> dict:
    return dict(map(lambda _: _.split(":"), list))

def isValid(p: dict) -> bool:
    a = "byr" in p
    b = "iyr" in p
    c = "eyr" in p
    d = "hgt" in p
    e = "hcl" in p
    f = "ecl" in p
    g = "pid" in p
    h = "cid" in p
    return (a and b and c and d and e and f and g)

def main():
    data = ao.data.split("\n\n")
    data = [string.replace("\n", " ") for string in data]
    data = [string.split() for string in data]
    data = [convListTodict(list) for list in data]

    valid_passports = sum(map(lambda x : isValid(x), data))

    print(f"Solution = {valid_passports}")
    #ao.submit(valid_passports)

if __name__ == '__main__':
    main()
