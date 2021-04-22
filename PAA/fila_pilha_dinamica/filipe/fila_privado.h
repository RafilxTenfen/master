#include "pilha_interface.h"

typedef struct NoFila {
  void *dados;
  struct NoFila *ant;
} NoFila;

typedef struct Fila {
  struct NoFila *inicio, *fim;
  int tamanho_dados;
  int tamanho;
} Fila;