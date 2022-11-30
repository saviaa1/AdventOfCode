#!/usr/bin/env python
import aocd as ao
import string

def convListTodict(list: list) -> dict:
    return dict(map(lambda _: _.split(":"), list))

def hasAllField(p: dict) -> bool:
    a = "byr" in p
    b = "iyr" in p
    c = "eyr" in p
    d = "hgt" in p
    e = "hcl" in p
    f = "ecl" in p
    g = "pid" in p
    h = "cid" in p
    return (a and b and c and d and e and f and g)

def checkHeightValid(height: str) -> bool:
    unit = height[-2:]
    if unit == "cm":
        if (str)(height[:-2]).isdigit():
            return 150 <= (int)(height[:-2]) <= 193
    elif unit == "in":
        if (str)(height[:-2]).isdigit():
            return 59 <= (int)(height[:-2]) <= 76
    return False

def checkHairColour(colour: str) -> bool:
    if colour[0] != "#" or len(colour) != 7: return False
    return set(colour[1:]) <= set(string.ascii_lowercase + string.digits)

def isValid(p: dict) -> bool:
    a = 1920 <= (int)(p["byr"]) <= 2002
    b = 2010 <= (int)(p["iyr"]) <= 2020
    c = 2020 <= (int)(p["eyr"]) <= 2030
    d = checkHeightValid(p["hgt"])
    e = checkHairColour(p["hcl"])
    f = p["ecl"] in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
    g = len(p["pid"]) == 9 and (str)(p["pid"]).isdigit()
    h = "cid" in p
    return (a and b and c and d and e and f and g)

def main():
    data = ao.data.split("\n\n")
    data = [string.replace("\n", " ") for string in data]
    data = [string.split() for string in data]
    data = [convListTodict(list) for list in data]

    field_valid_pass = list(filter(hasAllField, data))

    valid_passports = len(list(filter(isValid, field_valid_pass)))

    print(f"Solution = {valid_passports}")
    #ao.submit(valid_passports)

if __name__ == '__main__':
    main()
