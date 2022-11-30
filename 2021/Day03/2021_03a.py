#!/usr/bin/env python
import aocd as ao

def main():
    g_rate, e_rate = "", ""
 
    for i in range(12):
        one_c, zero_c = 0, 0
        for l in ao.lines:
            if l[i] == "1":
                one_c += 1
            elif l[i] == "0":
                zero_c += 1
 
        if one_c > zero_c:
            g_rate+="1"
            e_rate+="0"
        elif one_c < zero_c:
            g_rate+="0"
            e_rate+="1"
 
    mul = int(g_rate, 2) * int(e_rate, 2)
    print(f"Solution g: {g_rate} e: {e_rate} and g*e = {mul}") #738234
    #ao.submit(mul)

if __name__ == '__main__':
    main()