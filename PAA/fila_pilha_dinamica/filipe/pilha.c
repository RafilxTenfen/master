#include "pilha_privado.h"
#include "pilha_interface.h"

#include <stdlib.h>
#include <string.h>

#define FRACASSO 0
#define SUCESSO  1

int pilhaCriar(Pilha **pilha, int tamanho, int tamanho_dados) {
  if (tamanho_dados < 1 || pilha == NULL) return FRACASSO;

  *pilha = (Pilha*) malloc(sizeof(Pilha));

  (*pilha)->tamanho_dados = tamanho_dados;
  (*pilha)->tamanho_maximo = tamanho;
  (*pilha)->tamanho_atual = 0;

  (*pilha)->dados = (void**) malloc(tamanho * sizeof(void*));
  for (int i = 0; i < tamanho; ++i) {
    (*pilha)->dados[i] = malloc(tamanho_dados);
  }

  return SUCESSO; 
}

int pilhaDestruir(Pilha **pilha) {
  if (pilha == NULL || *pilha == NULL) return FRACASSO;

  for (int i = 0; i < (*pilha)->tamanho_maximo; ++i) {
    free((*pilha)->dados[i]);
  }
  free((*pilha)->dados);

  free(*pilha);

  *pilha = NULL;

  return SUCESSO;
}

int pilhaEmpilhar(Pilha *pilha, void *dados) {
  if (pilha == NULL || dados == NULL || pilhaCheia(pilha)) return FRACASSO;
  
  memcpy(pilha->dados[pilha->tamanho_atual], dados, pilha->tamanho_dados);

  pilha->tamanho_atual++;

  return SUCESSO;
}

int pilhaDesempilhar(Pilha *pilha, void *dados) {
  if (pilha == NULL || dados == NULL || pilhaVazia(pilha)) return FRACASSO;

  memcpy(dados, pilha->dados[pilha->tamanho_atual - 1], pilha->tamanho_dados);

  pilha->tamanho_atual--;

  return SUCESSO;
}

int pilhaTopo(Pilha *pilha, void *dados) {
  if (pilha == NULL || dados == NULL || pilhaVazia(pilha)) return FRACASSO;

  memcpy(dados, pilha->dados[pilha->tamanho_atual - 1], pilha->tamanho_dados);

  return SUCESSO;
}

int pilhaCheia(Pilha *pilha) {
  return pilha != NULL && pilha->tamanho_atual == pilha->tamanho_maximo;
}

int pilhaVazia(Pilha *pilha) {
  return pilha == NULL || pilha->tamanho_atual == 0;
}
