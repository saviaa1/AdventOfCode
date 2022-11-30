#!/usr/bin/env python
import aocd as ao
import numpy as np
from collections import Counter, defaultdict, deque
from queue import PriorityQueue
import sys

from collections import defaultdict
import sys
  
class Heap():
  
    def __init__(self):
        self.array = []
        self.size = 0
        self.pos = []
  
    def newMinHeapNode(self, v, dist):
        minHeapNode = [v, dist]
        return minHeapNode
  
    # A utility function to swap two nodes 
    # of min heap. Needed for min heapify
    def swapMinHeapNode(self, a, b):
        t = self.array[a]
        self.array[a] = self.array[b]
        self.array[b] = t
  
    # A standard function to heapify at given idx
    # This function also updates position of nodes 
    # when they are swapped.Position is needed 
    # for decreaseKey()
    def minHeapify(self, idx):
        smallest = idx
        left = 2*idx + 1
        right = 2*idx + 2
  
        if left < self.size and self.array[left][1] < self.array[smallest][1]:
            smallest = left
  
        if right < self.size and self.array[right][1] < self.array[smallest][1]:
            smallest = right
  
        # The nodes to be swapped in min 
        # heap if idx is not smallest
        if smallest != idx:
  
            # Swap positions
            self.pos[ self.array[smallest][0]] = idx
            self.pos[ self.array[idx][0]] = smallest
  
            # Swap nodes
            self.swapMinHeapNode(smallest, idx)
  
            self.minHeapify(smallest)
  
    # Standard function to extract minimum 
    # node from heap
    def extractMin(self):
  
        # Return NULL wif heap is empty
        if self.isEmpty() == True:
            return
  
        # Store the root node
        root = self.array[0]
  
        # Replace root node with last node
        lastNode = self.array[self.size - 1]
        self.array[0] = lastNode
  
        # Update position of last node
        self.pos[lastNode[0]] = 0
        self.pos[root[0]] = self.size - 1
  
        # Reduce heap size and heapify root
        self.size -= 1
        self.minHeapify(0)
  
        return root
  
    def isEmpty(self):
        return True if self.size == 0 else False
  
    def decreaseKey(self, v, dist):
  
        # Get the index of v in  heap array
  
        i = self.pos[v]
  
        # Get the node and update its dist value
        self.array[i][1] = dist
  
        # Travel up while the complete tree is 
        # not hepified. This is a O(Logn) loop
        while i > 0 and self.array[int(i)][1] < self.array[int((i - 1) // 2)][1]:
  
            # Swap this node with its parent
            self.pos[ self.array[i][0] ] = (i-1)//2
            self.pos[ self.array[(i-1)//2][0] ] = i
            self.swapMinHeapNode(i, (i - 1)//2 )
  
            # move to parent index
            i = (i - 1) // 2
  
    # A utility function to check if a given 
    # vertex 'v' is in min heap or not
    def isInMinHeap(self, v):
  
        if self.pos[v] < self.size:
            return True
        return False
  
  
def printArr(dist, n):
    print(f"Vertex\tDistance from source")
    for i in range(n):
        print(f"{i}\t\t{dist[i]}")
  
  
class Graph():
  
    def __init__(self, V):
        self.V = V
        self.graph = defaultdict(list)
  
    # Adds an edge to an undirected graph
    def addEdge(self, src, dest, weight):
  
        # Add an edge from src to dest.  A new node 
        # is added to the adjacency list of src. The 
        # node is added at the beginning. The first 
        # element of the node has the destination 
        # and the second elements has the weight
        newNode = [dest, weight]
        self.graph[src].insert(0, newNode)
  
        # Since graph is undirected, add an edge 
        # from dest to src also
        #newNode = [src, weight]
        #self.graph[dest].insert(0, newNode)
  
    # The main function that calculates distances 
    # of shortest paths from src to all vertices. 
    # It is a O(ELogV) function
    def dijkstra(self, src):
  
        V = self.V  # Get the number of vertices in graph
        dist = []   # dist values used to pick minimum 
                    # weight edge in cut
  
        # minHeap represents set E
        minHeap = Heap()
  
        #  Initialize min heap with all vertices. 
        # dist value of all vertices
        for v in range(V):
            dist.append(sys.maxsize)
            minHeap.array.append( minHeap.newMinHeapNode(v, dist[v]))
            minHeap.pos.append(v)
  
        # Make dist value of src vertex as 0 so 
        # that it is extracted first
        minHeap.pos[src] = src
        dist[src] = 0
        minHeap.decreaseKey(src, dist[src])
  
        # Initially size of min heap is equal to V
        minHeap.size = V
  
        # In the following loop, 
        # min heap contains all nodes
        # whose shortest distance is not yet finalized.
        while minHeap.isEmpty() == False:
  
            # Extract the vertex 
            # with minimum distance value
            newHeapNode = minHeap.extractMin()
            u = newHeapNode[0]
  
            # Traverse through all adjacent vertices of 
            # u (the extracted vertex) and update their 
            # distance values
            for pCrawl in self.graph[u]:
  
                v = pCrawl[0]
  
                # If shortest distance to v is not finalized 
                # yet, and distance to v through u is less 
                # than its previously calculated distance
                if minHeap.isInMinHeap(v) and dist[u] != sys.maxsize and pCrawl[1] + dist[u] < dist[v]:
                        dist[v] = pCrawl[1] + dist[u]
  
                        # update distance value 
                        # in min heap also
                        minHeap.decreaseKey(v, dist[v])
  
        #printArr(dist,V)
        return dist
        
def enlargeMap(data: list[list[int]]):
    #insert to right
    for x in range(len(data)):
        for i in range(0, len(data[x]) * 4):
            var = data[x][i]+1
            data[x].append(var if var <= 9 else 1)
    #insert to bottom and bottom right.
    for x in range(0, len(data) * 4):
        tempArr = []
        for y in range(0, len(data[x])):
            var = data[x][y]+1
            tempArr.append(var if var <= 9 else 1)
        data.append(tempArr)
    return data

def main():
    #data = ["1163751742","1381373672","2136511328","3694931569","7463417111","1319128137","1359912421","3125421639","1293138521","2311944581"]
    
    #data = ["11637517422274862853338597396444961841755517295286","13813736722492484783351359589446246169155735727126","21365113283247622439435873354154698446526571955763","36949315694715142671582625378269373648937148475914","74634171118574528222968563933317967414442817852555","13191281372421239248353234135946434524615754563572","13599124212461123532357223464346833457545794456865","31254216394236532741534764385264587549637569865174","12931385212314249632342535174345364628545647573965","23119445813422155692453326671356443778246755488935","22748628533385973964449618417555172952866628316397","24924847833513595894462461691557357271266846838237","32476224394358733541546984465265719557637682166874","47151426715826253782693736489371484759148259586125","85745282229685639333179674144428178525553928963666","24212392483532341359464345246157545635726865674683","24611235323572234643468334575457944568656815567976","42365327415347643852645875496375698651748671976285","23142496323425351743453646285456475739656758684176","34221556924533266713564437782467554889357866599146","33859739644496184175551729528666283163977739427418","35135958944624616915573572712668468382377957949348","43587335415469844652657195576376821668748793277985","58262537826937364893714847591482595861259361697236","96856393331796741444281785255539289636664139174777","35323413594643452461575456357268656746837976785794","35722346434683345754579445686568155679767926678187","53476438526458754963756986517486719762859782187396","34253517434536462854564757396567586841767869795287","45332667135644377824675548893578665991468977611257","44961841755517295286662831639777394274188841538529","46246169155735727126684683823779579493488168151459","54698446526571955763768216687487932779859814388196","69373648937148475914825958612593616972361472718347","17967414442817852555392896366641391747775241285888","46434524615754563572686567468379767857948187896815","46833457545794456865681556797679266781878137789298","64587549637569865174867197628597821873961893298417","45364628545647573965675868417678697952878971816398","56443778246755488935786659914689776112579188722368","55172952866628316397773942741888415385299952649631","57357271266846838237795794934881681514599279262561","65719557637682166874879327798598143881961925499217","71484759148259586125936169723614727183472583829458","28178525553928963666413917477752412858886352396999","57545635726865674683797678579481878968159298917926","57944568656815567976792667818781377892989248891319","75698651748671976285978218739618932984172914319528","56475739656758684176786979528789718163989182927419","67554889357866599146897761125791887223681299833479"]
    
    data = ao.lines
    data = [[int(i) for i in row] for row in data]
    
    data = enlargeMap(data)

    assert len(data) == len(data[0])

    G = Graph(len(data)**2)

    maxx = len(data[0])
    maxy = len(data)
    for x in range(maxx):
        for y in range(maxy):
            start_edge = x + maxx * y
            for (dx, dy) in [(x+1, y), (x-1, y), (x, y+1), (x, y-1)]:
                if 0 <= dx < maxx and 0 <= dy < maxy:
                    end_edge = dx + maxx * dy
                    weight = data[dx][dy]
                    #print(start_edge, end_edge, weight)
                    G.addEdge(start_edge, end_edge, weight)

    res = G.dijkstra(0)

    print(f"Solution for part1 = {res[-1:]}")
    #ao.submit(3587)

    res = 0

    print(f"Solution for part2 = {res}")
    #ao.submit(3906445077999)

if __name__ == '__main__':
    main()
