#ifndef __PILHA_H__
#define __PILHA_H__

#include <stdbool.h>

typedef struct pilha pilha_t;

pilha_t *Init();

int Push(pilha_t *pilha, void *elemento);

int Pop(pilha_t *pilha);

bool IsEmpty(pilha_t *pilha);

int Size(pilha_t *pilha);

void *Top(pilha_t *pilha);

void Destroy(pilha_t *pilha);

#endif /* __PILHA_H__ */