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

### InsertionSort
# http://python3.codes/popular-sorting-algorithms/
def insertion_sort(l):
    for i in range(1, len(l)):
        j = i-1
        key = l[i]
        while (l[j] > key) and (j >= 0):
            l[j+1] = l[j]
            j -= 1
        l[j+1] = key


### SelectionSort
# http://python3.codes/popular-sorting-algorithms/
def selection_sort(lst):
    for i, e in enumerate(lst):
        mn = min(range(i,len(lst)), key=lst.__getitem__)
        lst[i], lst[mn] = lst[mn], e
    return lst


### programa principal

n = 16000                     # tamanho do vetor
v = [i for i in range(n)]     # vetor inicial ordenado de 0 a n-1
if len(sys.argv) > 1:
    nreps = int(sys.argv[1])  # numero de repeticoes
else:
    nreps = 3                 # numero default de repeticoes
random.seed()
print("insertion  selection")
for k in range(1, nreps+1):
    random.shuffle(v)
    v2 = v[:]
    tini = time.time()
    insertion_sort(v)
    t1 = time.time() - tini
#    sys.stderr.write("k=%d - InsertionSort done\n" % (k))
    tini = time.time()
    selection_sort(v2)
    t2 = time.time() - tini
#    sys.stderr.write("k=%d - SelectionSort done\n" % (k))
    print("%.5f  %.5f" % (t1, t2))
