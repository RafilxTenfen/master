/* pilha_privado.h */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "pilha_publico.h"

enum retornos {
  FALSE,
  TRUE,
  ESTRUTURA_NULLA,
  ESTRUTURA_SEM_DADOS,
  ESTRUTURA_LOTADA
};

struct PE {
  int topo;
  int tmax;
  int tinfo;
  void **dados;
};

int cheia(pPilha p);
int vazia(pPilha p);