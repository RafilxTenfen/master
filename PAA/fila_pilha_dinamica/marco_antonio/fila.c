#include "fila_privado.h"
#include "fila_interface.h"

#include <stdlib.h>
#include <string.h>

#define FRACASSO 0
#define SUCESSO  1

int filaCriar(Fila **fila, int tamanho_dados) {
  if (fila == NULL || tamanho_dados < 1) return FRACASSO;

  *fila = (Fila*) malloc(sizeof(Fila));
  (*fila)->tamanho_dados = tamanho_dados;
  (*fila)->tamanho = 0;
  (*fila)->inicio = (*fila)->fim = NULL;

  return SUCESSO;
}

int filaDestruir(Fila **fila) {
  if (fila == NULL || *fila == NULL) return FRACASSO;

  void *dados = malloc((*fila)->tamanho_dados);
  while ((*fila)->inicio != NULL) {
    filaDesenfileirar(*fila, dados);
  }
  free(dados);

  free(*fila);

  return SUCESSO;
}

int filaEnfileirar(Fila *fila, void *dados) {
  if (fila == NULL || dados == NULL) return FRACASSO;

  NoFila *novo_no = (NoFila*) malloc(sizeof(NoFila));
  novo_no->dados = malloc(fila->tamanho_dados);
  memcpy(novo_no->dados, dados, fila->tamanho_dados);
  novo_no->ant = NULL;

  if (fila->inicio == NULL) {
    fila->inicio = fila->fim = novo_no;
  } else {
    fila->fim->ant = novo_no;
    fila->fim = novo_no;
  }

  fila->tamanho++;

  return SUCESSO;
}

int filaDesenfileirar(Fila *fila, void *dados) {
  if (fila == NULL || fila->inicio == NULL) return FRACASSO;

  NoFila *no = fila->inicio;
  
  memcpy(dados, no->dados, fila->tamanho_dados);
  free(no->dados);

  if (no->ant == NULL) {
    fila->inicio = fila->fim = NULL;
  } else {
    fila->inicio = no->ant;
  }

  free(no);

  fila->tamanho--;

  return SUCESSO;
}

int filaCabeca(Fila *fila, void *dados) {
  if (fila == NULL || fila->inicio == NULL) return FRACASSO;

  memcpy(dados, fila->inicio->dados, fila->tamanho_dados);

  return SUCESSO; 
}

int filaCauda(Fila *fila, void *dados) {
  if (fila == NULL || fila->fim == NULL) return FRACASSO;

  memcpy(dados, fila->fim->dados, fila->tamanho_dados);

  return SUCESSO; 
}

int filaTamanho(Fila *fila) {
  if (fila == NULL) return 0;

  return fila->tamanho;
}