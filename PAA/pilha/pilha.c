/* pilha.c */

#include "pilha_privado.h"
#include "pilha_publico.h"

int criaPilha(ppPilha pp, int tamanhoVetor, int tamanhoInfo) {
  pp = malloc(sizeof(ppPilha));

  (*pp)->dados = malloc(tamanhoInfo * tamanhoVetor);
  (*pp)->topo = 0;
  (*pp)->tinfo = tamanhoInfo;
  (*pp)->tmax = tamanhoVetor;

  return TRUE;
};

int destroiPilha(ppPilha pp) {

};

int empilha(pPilha p, void *elemento) {
  if (p == NULL) {
    return ESTRUTURA_NULLA;
  }
  if (cheia(p) != FALSE) {
    return ESTRUTURA_LOTADA;
  }
  p->dados[p->topo] = elemento;
  p->topo = p->topo + 1;
};

int desempilha(pPilha p, void *elemento) {

};

int reiniciaPilha(pPilha p){

};

int topo(pPilha p, void *topo) {
  if (p == NULL) {
    return ESTRUTURA_NULLA;
  }
  if (vazia(p)) {
    return ESTRUTURA_SEM_DADOS;
  }
  memcpy(topo, p->dados, p->tinfo);
  return TRUE;
};

int cheia(pPilha p) {
  if (p == NULL) {
    return ESTRUTURA_NULLA;
  }
  if (p->topo == p->tmax) {
    return TRUE;
  }
  return FALSE;
};

int vazia(pPilha p) {
  if (p == NULL) {
    return ESTRUTURA_NULLA;
  }
  if (p->topo == 0) {
    return TRUE;
  }
  return FALSE;
};