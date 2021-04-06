#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "Fila_privada.h"
#include "Fila_publica.h"

#define TRUE 5
#define FALSE 4

void printValue(char *prefix, void *valorAlocado) {
  printf("\n%s %p -> %s", prefix, valorAlocado, (char *)valorAlocado);
}

void printFila(Fila *f) {
  if (f == NULL) {
    return;
  }
  if (f->inicio != NULL && f->inicio->dados != NULL) {
    printValue("inicio dados", f->inicio->dados);
    if (f->inicio->ant != NULL && f->inicio->ant->dados != NULL) {
      printValue("inicio ant dados", f->inicio->ant->dados);
    }
  }

  if (f->fim != NULL && f->fim->dados != NULL) {
    printValue("fim dados", f->fim->dados);
    if (f->fim->ant != NULL && f->fim->ant->dados != NULL) {
      printValue("fim ant dados", f->fim->ant->dados);
    }
  }
}

Fila *criar(int tamanho_dados, int *resultado) {
  Fila *f = (Fila *)malloc(sizeof(Fila));
  if (f == NULL) {
    *resultado = ERRO;
    return NULL;
  }

  f->tamanho_dados = tamanho_dados;
  f->inicio = NULL;
  f->fim = NULL;

  *resultado = SUCCESSO;

  return f;
};

void enfileirar(Fila *f, void *elemento, int *resultado) {
  extern char _etext;
  if (f == NULL || elemento == NULL || !elemento || (char*) elemento < &_etext) {
    *resultado = ERRO;
    return;
  }

  NoFila *no = (NoFila *)malloc(sizeof(Fila));
  if (no == NULL) {
    *resultado = ERRO;
    return;
  }
  no->dados = malloc(f->tamanho_dados);
  if (no->dados == NULL) {
    *resultado = ERRO;
    return;
  }

  resultado = SUCCESSO;
  no->ant = NULL;
  memcpy(no->dados, elemento, f->tamanho_dados);

  if (vazia(f) == TRUE || cheia(f) == TRUE) {
    f->inicio = no;
  } else {             // contains something in the slice
    f->fim->ant = no;  // receive next, because the end is the pointer of the previous
  }
  f->fim = no;
  // printFila(f);
};

void *desenfileirar(Fila *f, int *resultado) {
  if (f == NULL) {
    *resultado = ERRO;
    return NULL;
  }

  if (vazia(f) == TRUE || cheia(f) == TRUE) {
    *resultado = ERRO;
    return NULL;
  }

  // printFila(f);
  void *dados = malloc(f->tamanho_dados);

  NoFila *current = f->inicio;
  memcpy(dados, current->dados, f->tamanho_dados);
  f->inicio = current->ant;

  if (f->inicio == NULL) {
    f->fim = NULL;
  }
  if (current != NULL) {
    if (current->dados != NULL) {
      free(current->dados);
    }
    free(current);
  }

  *resultado = SUCCESSO;
  // printFila(f);
  return dados;
};

void destruir(Fila *f, int *resultado) {
  *resultado = SUCCESSO;
  if (f == NULL) {
    return;
  }

  while (f->inicio != NULL) {
    NoFila* current = f->inicio;
    if (!current->ant->dados) {
      free(current->ant->dados);
    }
    if (!current->ant) {
      free(current->ant);
    }
    f->inicio = f->inicio->ant;
    if (current->dados != NULL) {
      free(current->dados);
    }
    free(current);
  }
};

int cheia(Fila *f) {
  int result = FALSE;
  void *p = malloc(f->tamanho_dados);
  if (f == NULL) {
    result = TRUE;
  }
  free(p);

  return result;
}

int vazia(Fila *f) {
  if (f == NULL) {
    return TRUE;
  }
  if (f->inicio == NULL) {
    return TRUE;
  }
  if (f->fim == NULL) {
    return TRUE;
  }
  return FALSE;
}