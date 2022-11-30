#!/usr/bin/env python
import aocd as ao
import numpy as np

def runStates(state: np.ndarray, day: int) -> np.ndarray:
    for _ in range(day):
        state0 = state[0]
        for i in range(0, 8):
            state[i] = state[i+1]
        state[6] += state0
        state[8] = state0
    return state


def main():
    data = np.array(ao.data.split(","), dtype=np.int64)

    states = np.zeros(9, dtype=np.int64)

    # init states:
    for i in data:
        states[i] += 1

    states = runStates(states, 80)
    assert np.sum(states) == 352151, f"size: {np.sum(states)}"

    states = runStates(states, 256-80)

    sum = np.sum(states)
    print(f"Solution = {sum}")
    #ao.submit(sum)

if __name__ == '__main__':
    main()