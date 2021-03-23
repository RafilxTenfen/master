#include "pilha_privado.h"
#include "pilha_interface.h"

#include <stdlib.h>
#include <string.h>

#define FRACASSO 0
#define SUCESSO  1

int criarPilha(Pilha **pilha, int tamanho_dado) {
  if (tamanho_dado < 1) return FRACASSO;
  if (pilha == NULL) return FRACASSO;

  *pilha = (Pilha*) malloc(sizeof(Pilha));

  (*pilha)->tamanho_dado = tamanho_dado;
  (*pilha)->tamanho = 0;
  (*pilha)->topo = NULL;

  return SUCESSO; 
}

int destruirPilha(Pilha **pilha) {
  if (pilha == NULL) return FRACASSO;
  if (*pilha == NULL) return FRACASSO;

  while ((*pilha)->topo != NULL) {
    desempilharPilha(*pilha);
  }

  free(*pilha);

  *pilha = NULL;

  return SUCESSO;
}

int empilharPilha(Pilha *pilha, void *dado) {
  if (pilha == NULL) return FRACASSO;
  if (dado == NULL) return FRACASSO;

  PilhaDado *novo_dado = (PilhaDado*) malloc(sizeof(PilhaDado));
  novo_dado->dado = malloc(pilha->tamanho_dado);
  memcpy(novo_dado->dado, dado, pilha->tamanho_dado);

  novo_dado->proximo = pilha->topo;
  pilha->topo = novo_dado;
  pilha->tamanho++;

  return SUCESSO;
}

int desempilharPilha(Pilha *pilha) {
  if (pilha == NULL) return FRACASSO;
  if (pilha->topo == NULL) return FRACASSO;

  PilhaDado *topo_atual = pilha->topo;

  pilha->topo = topo_atual->proximo;

  free(topo_atual->dado);
  free(topo_atual);

  pilha->tamanho--;

  return SUCESSO;
}

int topoPilha(Pilha *pilha, void *dado) {
  if (pilha == NULL) return FRACASSO;
  if (pilha->topo == NULL) return FRACASSO;
  if (dado == NULL) return FRACASSO;

  memcpy(dado, pilha->topo->dado, pilha->tamanho_dado);

  return SUCESSO;
}

int tamanhoPilha(Pilha *pilha, int *tamanho) {
  if (pilha == NULL) return FRACASSO;
  
  *tamanho = pilha->tamanho;

  return SUCESSO;
}
