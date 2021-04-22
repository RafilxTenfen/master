#include "fila_pilha_privado.h"
#include "fila_pilha_interface.h"
#include "fila_interface.h"
#include "pilha_interface.h"

#include <stdlib.h>
#include <string.h>

#define FRACASSO 0
#define SUCESSO  1

int filaPilhaCriar(FilaPilha **fila_pilha, int tamanho_pilha, int tamanho_dados) {
  if (fila_pilha == NULL || tamanho_dados < 1) return FRACASSO;


  (*fila_pilha) = (FilaPilha*) malloc(sizeof(FilaPilha));

  filaCriar(&(*fila_pilha)->fila, sizeof(Pilha*));
  
  (*fila_pilha)->tamanho_pilha = tamanho_pilha;
  (*fila_pilha)->tamanho_dados = tamanho_dados;

  return SUCESSO;
}

int filaPilhaDestruir(FilaPilha **fila_pilha) {
  if (fila_pilha == NULL || *fila_pilha == NULL) return FRACASSO;

  void *dados = malloc((*fila_pilha)->tamanho_dados);
  while (!filaPilhaVazia(*fila_pilha)) {
    filaPilhaRemover(*fila_pilha, dados);
  }
  free(dados);

  filaDestruir(&(*fila_pilha)->fila);

  free(*fila_pilha);

  *fila_pilha = NULL;

  return SUCESSO;
}

int filaPilhaInserir(FilaPilha *fila_pilha, void *dados) {
  if (fila_pilha == NULL || dados == NULL) return FRACASSO;

  Pilha *pilha;

  int existePilha = filaCauda(fila_pilha->fila, &pilha);
  if (!existePilha || pilhaCheia(pilha)) {
    pilhaCriar(&pilha, fila_pilha->tamanho_pilha, fila_pilha->tamanho_dados);
    filaEnfileirar(fila_pilha->fila, &pilha);
  }

  pilhaEmpilhar(pilha, dados);

  return SUCESSO;
}

int filaPilhaRemover(FilaPilha *fila_pilha, void *dados) {
  if (fila_pilha == NULL || dados == NULL || filaPilhaVazia(fila_pilha)) return FRACASSO;

  Pilha *pilha;
  filaCabeca(fila_pilha->fila, &pilha);
  pilhaDesempilhar(pilha, dados);

  if (pilhaVazia(pilha)) {
    filaDesenfileirar(fila_pilha->fila, &pilha);
    pilhaDestruir(&pilha);
  }

  return SUCESSO;
}

int filaPilhaVazia(FilaPilha *fila_pilha) {
  return fila_pilha == NULL || filaTamanho(fila_pilha->fila) == 0;
}