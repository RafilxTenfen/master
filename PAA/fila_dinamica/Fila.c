#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include "Fila_publica.h"
#include "Fila_privada.h"

void printFila(Fila *f) {
  int *valorAlocado = malloc(sizeof(int *));
  valorAlocado = f->inicio->dados;
  printf("\ninicio dados %p -> %d", f->inicio->dados, *valorAlocado);
  valorAlocado = f->fim->dados;
  printf("\nfim dados %p -> %d", f->fim->dados, *valorAlocado);
}

Fila* criar(int tamanho_dados, int *resultado) {
  Fila* f = (Fila *) malloc(sizeof(Fila));

  f->tamanho_dados = tamanho_dados;
  f->inicio = NULL;
  f->fim = NULL;

  *resultado = TRUE;

  return f;
};

void enfileirar(Fila *f, void *elemento, int *resultado) {
  if (f == NULL) {
    *resultado = FALSE;
    return;
  }

  NoFila* no = (NoFila *) malloc(sizeof(Fila));
  no->dados = malloc(f->tamanho_dados);
  no->ant = NULL;
  memcpy(no->dados, elemento, f->tamanho_dados);

  if (vazia(f) == TRUE) {
    printf("\n vazio == true");
    f->inicio = no;
    f->fim = no;
  } else {
    NoFila* current = f->fim;
    no->ant = current;
    f->fim = no;
  }

  printFila(f);
  resultado = TRUE;
};

void* desenfileirar(Fila *f, int *resultado) {
  if (f == NULL) {
    *resultado = FALSE;
    return 0;
  }

  NoFila* current = f->fim;
  f->fim = current->ant;

  return current->dados;
};

void destruir(Fila *f, int *resultado) {

};

int cheia(Fila* f) {
  return FALSE;
}

int vazia(Fila* f) {
  if (f == NULL) {
    return TRUE;
  }
  if (f->inicio == NULL) {
    return TRUE;
  }
  return FALSE;
}