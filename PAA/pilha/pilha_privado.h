/* pilha_privado.h */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define TRUE 1
#define FALSE 0

#define ESTRUTURA_NULLA -5
#define ESTRUTURA_SEM_DADOS -6
#define ESTRUTURA_LOTADA -7

#include "pilha_publico.h"

struct PE {
  int topo;
  int tmax;
  int tinfo;
  void **dados;
};

int cheia(pPilha p);
int vazia(pPilha p);