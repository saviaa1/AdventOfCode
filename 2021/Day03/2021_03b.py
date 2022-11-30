#!/usr/bin/env python
import aocd as ao
import numpy as np

def calulateVV(i: int, data: np.ndarray, ifOneOverZero: bool):
    one_c, zero_c = 0, 0
    for r in data:
        if r[i] == "1":
            one_c += 1
        elif r[i] == "0":
            zero_c += 1
    if  one_c >= zero_c:
        return np.vectorize(lambda x: x[i] == ("1" if ifOneOverZero else "0"))
    return np.vectorize(lambda x: x[i] == ("0" if ifOneOverZero else "1"))

def main():
    data = np.array(ao.lines, dtype="U25")
 
    #ox
    ox_data = data
    for i in range(12):
        vv = calulateVV(i, ox_data, True)
        ox_data = ox_data[np.where(vv(ox_data))]
        if ox_data.size <= 1:
            break
 
    #co2
    co2_data = data
    for i in range(12):
        vv = calulateVV(i, co2_data, False)
        co2_data = co2_data[np.where(vv(co2_data))]
        if co2_data.size <= 1:
            break
 
    mul = int(ox_data[0], 2) * int(co2_data[0], 2)
    print(f"Solution ox = {ox_data}, co2 = {co2_data},  mul = {mul}")
    #ao.submit(mul)

if __name__ == '__main__':
    main()