#! /usr/bin/env python

# script para medir o tempo de execucao de algoritmos de ordenacao
# sobre os mesmos vetores. Tempos reportados em segundos.
#
# Para ajustar o tempo de execucao e' possivel calibrar 'n' (tamanho
# do vetor) no programa principal
#
# O numero de repeticoes pode ser passado como parametro na linha de
# comando

import random, sys, time

### MergeSort
# http://python3.codes/popular-sorting-algorithms/
from heapq import merge

def merge_sort(m):
    if len(m) <= 1:
        return m

    middle = len(m) // 2
    left = m[:middle]
    right = m[middle:]

    left = merge_sort(left)
    right = merge_sort(right)
    return list(merge(left, right))

### HeapSort
# http://python3.codes/popular-sorting-algorithms/
def swap(list, i, j):
    list[i], list[j] = list[j], list[i]

def heapify(list, end,i):
    l=2 * i + 1
    r=2 * (i + 1)
    max=i
    if l < end and list[i] < list[l]:
        max = l
    if r < end and list[max] < list[r]:
        max = r
    if max != i:
        swap(list, i, max)
        heapify(list, end, max)

def heap_sort(list):
    end = len(list)
    start = end // 2 - 1
    for i in range(start, -1, -1):
        heapify(list, end, i)
    for i in range(end-1, 0, -1):
        swap(list, i, 0)
        heapify(list, i, 0)

### QuickSort
# http://python3.codes/popular-sorting-algorithms/

def quickSort(arr):
    less = []
    pivotList = []
    more = []
    if len(arr) <= 1:
        return arr
    else:
        pivot = arr[0]
        for i in arr:
            if i < pivot:
                less.append(i)
            elif i > pivot:
                more.append(i)
            else:
                pivotList.append(i)
        less = quickSort(less)
        more = quickSort(more)
        return less + pivotList + more


### programa principal

n = 80000                    # tamanho do vetor
v = [i for i in range(n)]     # vetor inicial ordenado de 0 a n-1
if len(sys.argv) > 1:
    nreps = int(sys.argv[1])  # numero de repeticoes
else:
    nreps = 3                 # numero default de repeticoes
random.seed()
print("merge\theap\tquick")
for k in range(1, nreps+1):
    random.shuffle(v)
    v2 = v[:]
    random.shuffle(v2)
    v3 = v[:]
    random.shuffle(v3)
    tini = time.time()
    v = merge_sort(v)
    t1 = time.time() - tini
#    sys.stderr.write("k=%d - MergeSort done\n" % (k))
    tini = time.time()
    heap_sort(v2)
    t2 = time.time() - tini
#    sys.stderr.write("k=%d - HeapSort done\n" % (k))
    tini = time.time()
    v3 = quickSort(v3)
    t3 = time.time() - tini
#    sys.stderr.write("k=%d - QuickSort done\n" % (k))
    print("%.5f\t%.5f\t%.5f" % (t1, t2, t3))
