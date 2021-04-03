#ifndef _fila_privado
#define _fila_privado

typedef struct NoFila {
  void *dados;
  struct NoFila *ant;
} NoFila;

typedef struct Fila {
  NoFila *inicio, *fim;
  int tamanho_dados;
} Fila;

int cheia(Fila* f);
int vazia(Fila* f);

#endif