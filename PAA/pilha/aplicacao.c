
#include <stdio.h>
#include <stdlib.h>
#include "pilha_publico.h"

int main(int argc, char **argv) {
  printf("\n INIT MAIN");

  ppPilha myPilha = NULL;

  printf("\n vai criar pilha");
  criaPilha(myPilha, 4, sizeof(int));
  printf("\n criou pilha");

  int elemento = 100;
  empilha(*myPilha, &elemento);

  int intTopo;
  topo(*myPilha, &intTopo);
  printf("topo %i", intTopo);

  return EXIT_SUCCESS;
}