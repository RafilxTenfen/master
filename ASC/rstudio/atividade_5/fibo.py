#! /usr/bin/env python3

# Script que mede o tempo necessario para encontrar N vezes o 50o termo
# da sequencia de Fibonacci. Tempos reportados em segundos.
#
# Tres argumentos sao passados na linha de comando: os valores inicial e
# final de N, e o passo de incremento. Por exemplo, "10 50 10" faz
# N = { 10, 20, 30, 40, 50 }
# Para um unico valor de N, basta fazer inicial=final
#
# Um quarto argumento opcional e' o numero de repeticoes para cada valor de N

import sys, time

### programa principal

if len(sys.argv) < 3:
    sys.stderr.write("uso: %s N-inicial N-final passo [nrep]\n" %
                     sys.argv[0])
    sys.exit(1)
if len(sys.argv) > 4:
    nreps = int(sys.argv[4])  # numero de repeticoes
else:
    nreps = 1                 # numero default de repeticoes
inicio = int(sys.argv[1])     # N inicial
fim = int(sys.argv[2])        # N final
passo = int(sys.argv[3])      # passo de incremento
if (inicio > fim or passo <= 0):
    sys.stderr.write("erro: argumentos invalidos\n")
    sys.stderr.write("uso: %s N-inicial N-final passo [nrep]\n" %
                     sys.argv[0])
    sys.exit(1)
    
print("n\ttempo")
for n in range(inicio, fim+1, passo):
    for k in range(1, nreps+1):
        tini = time.time()
        for j in range(n):
            (n1, n2) = (1, 1)
            i = 2
            while (i < 50):
                i = i + 1
                n3 = n1 + n2
                (n1, n2) = (n2, n3)
        t1 = time.time() - tini
        print("%d\t%.5f" % (n, t1))
