#ifndef GUARD_PILHA_PRIVADO
#define GUARD_PILHA_PRIVADO

typedef struct PilhaDado{
  void *dado;
  struct PilhaDado *proximo;
} PilhaDado;

typedef struct Pilha {
  PilhaDado *topo;
  int tamanho_dado;
  int tamanho;
} Pilha;

#endif