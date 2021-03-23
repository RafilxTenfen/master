#ifndef GUARD_PILHA_INTERFACE
#define GUARD_PILHA_INTERFACE

typedef struct Pilha Pilha;

int criarPilha(Pilha **pilha, int tamanho_dado);
int destruirPilha(Pilha **pilha);

int empilharPilha(Pilha *pilha, void *dado);
int desempilharPilha(Pilha *pilha);

int topoPilha(Pilha *pilha, void *dado);
int tamanhoPilha(Pilha *pilha, int *tamanho);

#endif