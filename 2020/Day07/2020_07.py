#!/usr/bin/env python
import aocd as ao
import networkx as nx
import matplotlib.pyplot as plt
from collections import defaultdict
import re

def part1(G: nx.DiGraph) -> int:
    #nx.draw(G)
    #plt.show()
    RG = G.reverse()

    predecessors2 = nx.dfs_predecessors(RG, "shiny gold")
    return len(predecessors2)

def part2(G: nx.DiGraph) -> int:
    def search(node: str):
        count = 1
        for n in G.neighbors(node):
            count += G[node][n]["weight"] * search(n)
        return count
    return search("shiny gold") - 1

def main():
    data = ao.lines
    bags = defaultdict(dict)
    for l in data:
        bag = re.match(r"(.*) bags contain", l).groups()[0]
        for count, b in re.findall(r"(\d+) (\w+ \w+) bag", l):
            bags[bag][b] = { 'weight': int(count) }

    G = nx.DiGraph(bags)

    res = part1(G)
    print(f"Solution part1 = {res}")
    #ao.submit(res)

    res = part2(G)
    print(f"Solution part2 = {res}")
    #ao.submit(res)

if __name__ == '__main__':
    main()