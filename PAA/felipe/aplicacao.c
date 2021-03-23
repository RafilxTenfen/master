
#include <stdio.h>
#include <stdlib.h>
#include "pilha_interface.h"

int main(int argc, char **argv) {
  Pilha* p;
  destruirPilha(p);
  criarPilha(&p, sizeof(int));

  desempilharPilha(p);

  int element1 = 43;
  empilharPilha(p, &element1);

  desempilharPilha(p);
  int elementTop;
  topoPilha(p, &elementTop);
  printf("Valor topo %d", elementTop);

  int tamanho;
  tamanhoPilha(p, &tamanho);
  printf("\nTamanho %d", tamanho);

  destruirPilha(p);


}