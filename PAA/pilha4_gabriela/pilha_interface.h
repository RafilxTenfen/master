#ifndef _pilha
#define _pilha
#include <stdio.h>
#include <stdlib.h>

typedef struct stack_t *pPilha, **ppPilha;

int is_full(pPilha stack);
int is_empty(pPilha stack);

void *top(pPilha stack);

ppPilha init(int data_size);
void destroy(ppPilha stack);

void push(pPilha stack, void *element);
void *pop(pPilha stack);

#endif