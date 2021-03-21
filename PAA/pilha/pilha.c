/* pilha.c */
#include <stdio.h>
#include <stdlib.h>

#include "pilha_privado.h"
#include "pilha_publico.h"

int printErrorEnum(enum retornos ret) {
  switch (ret) {
    case FALSE:
      printf("\nA função foi invocada retornou falso");
      break;

    case ESTRUTURA_NULLA:
      printf("\nA estrutura utilizada esta nulla");
      break;

    case ESTRUTURA_SEM_DADOS:
      printf("\nA estrutura esta vazia");
      break;

    case ESTRUTURA_LOTADA:
      printf("\nA estrutura esta cheia");
      break;

    case TRUE:
      printf("\nA função foi invocada e retornou true");
      break;

    default:
      printf("\nEnum não identificado");
      break;
  }
  return ret;
}

int isPilhaNull(pPilha p) {
  if (p == NULL || p->tinfo < 0) {
    return TRUE;
  }
  return FALSE;
}

int criaPilha(ppPilha pp, int tamanhoVetor, int tamanhoInfo) {
  (*pp) = malloc(sizeof(struct PE));
  if (pp == NULL) {
    printf("Voce deve instanciar pPilha p; e passar como parametro criaPilha(&p, ...)");
    return printErrorEnum(ESTRUTURA_NULLA);
  }
  if (tamanhoInfo <= 0 || tamanhoVetor <= 0) {
    printf("Os valores de Tamanho info %d e Tamanho vetor %d devem ser > 0", tamanhoInfo, tamanhoVetor);
    return printErrorEnum(ESTRUTURA_NULLA);
  }

  (*pp)->dados = malloc(tamanhoInfo * tamanhoVetor);
  (*pp)->topo = 0;
  (*pp)->tinfo = tamanhoInfo;
  (*pp)->tmax = tamanhoVetor;

  return TRUE;
};

int destroiPilha(ppPilha pp) {
  if (pp == NULL || (*pp) == NULL || (*pp)->tinfo < 0) {
    printf("\nVoce esta tentnado destruindo uma estrutura nulla");
    return printErrorEnum(ESTRUTURA_NULLA);
  }
  free((*pp)->dados);
  free(*pp);
  (*pp) = NULL;
  return TRUE;
};

int empilha(pPilha p, void *elemento) {
  int ret = cheia(p);
  if (ret != FALSE) {
    return printErrorEnum(ret);
  }
  p->dados[p->topo] = elemento;
  p->topo++;
  return TRUE;
};

int desempilha(pPilha p, void *elemento) {
  if (isPilhaNull(p) == TRUE) {
    return printErrorEnum(ESTRUTURA_NULLA);
  }

  if (p->topo <= 0) {
    printf("Estrutura ja esta sem dados para desempilhar");
    return printErrorEnum(ESTRUTURA_SEM_DADOS);
  }

  int ret = topo(p, elemento);
  if (ret != TRUE) {
    return printErrorEnum(ret);
  }

  p->topo--;
  return TRUE;
};

int reiniciaPilha(pPilha p) {
  if (p == NULL) {
    return printErrorEnum(ESTRUTURA_NULLA);
  }
  p->topo = 0;
  free(p->dados);
  p->dados = malloc(p->tinfo * p->tmax);
  return TRUE;
};

int topo(pPilha p, void *topo) {
  int ret = vazia(p);
  if (ret) {
    return printErrorEnum(ret);
  }
  memcpy(topo, p->dados[p->topo - 1], p->tinfo);
  return TRUE;
};

int cheia(pPilha p) {
  if (isPilhaNull(p) == TRUE) {
    return ESTRUTURA_NULLA;
  }
  if (p->topo == p->tmax) {
    return ESTRUTURA_LOTADA;
  }
  return FALSE;
};

int vazia(pPilha p) {
  if (isPilhaNull(p) == TRUE) {
    return ESTRUTURA_NULLA;
  }
  if (p->topo != 0) {
    return FALSE;
  }
  return TRUE;
};

