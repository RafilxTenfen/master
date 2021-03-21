
#include <stdio.h>
#include <stdlib.h>
#include "pilha_publico.h"

int main(int argc, char **argv) {
  int nElementos = 4;
  pPilha p;

  // int elemento3;
  // desempilha(p, &elemento3);
  // printf("\n desempilha %d", elemento3);

  // int intTopo;
  // topo(p, &intTopo);
  // printf("\n topo %d", intTopo);

  criaPilha(&p, nElementos, sizeof(int));

  int elemento = 7;
  printf("\n insere %d", elemento);
  empilha(p, &elemento);

  int elemento2 = 12;
  printf("\n insere %d", elemento2);
  empilha(p, &elemento2);

  int intTopo;
  topo(p, &intTopo);
  printf("\n topo %d", intTopo);

  int elemento3;
  desempilha(p, &elemento3);
  printf("\n desempilha %d", elemento3);
  int elemento98;
  desempilha(p, &elemento98);
  printf("\n desempilha %d", elemento98);

  topo(p, &intTopo);
  printf("\n topo %d", intTopo);

  int elemento4 = 99;
  printf("\n insere %d", elemento4);
  empilha(p, &elemento4);

  printf("\n reinicia a pilha");
  reiniciaPilha(p);

  int elemento5 = 99;
  printf("\n insere %d", elemento5);
  empilha(p, &elemento5);

  printf("\n destroi a pilha");
  destroiPilha(&p);

  int elemento6 = 6;
  printf("\n insere %d", elemento6);
  empilha(p, &elemento6);

  int elemento7 = 7;
  printf("\n insere %d", elemento7);
  empilha(p, &elemento7);
  printf("\n insere %d", elemento4);
  empilha(p, &elemento4);

  int intTopo2;
  topo(p, &intTopo2);
  printf("\n topo %d", intTopo2);

  return EXIT_SUCCESS;
}