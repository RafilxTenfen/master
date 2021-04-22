/* pilha.c */
#include <stdio.h>
#include <stdlib.h>
#include "Pilha_privado.h"
#include "Pilha_interface.h"

struct Stack *createStack(int tamItem)
{
    struct Stack *stack = (struct Stack *)malloc(sizeof(struct Stack));
    stack->tamItem = tamItem;
    stack->topo = 0;
    printf("\nA pilha foi criada (%p)\n", stack);
    return stack;
}

int estaCheio(struct Stack *stack)
{
    return stack->topo == PILHA_TAM_MAX;
}

int estaVazio(struct Stack *stack)
{
    return stack->topo <= 0;
}

void push(struct Stack *stack, void *item)
{
    if (estaCheio(stack))
        return;

    memcpy(&stack->itens[stack->topo], &item, stack->tamItem);
    stack->topo++;

    printf("\nO Elemento %d foi empilhado\n", item);
}

void *pop(struct Stack *stack)
{
    if (estaVazio(stack))
        return 0;

    void *elremovido = malloc(stack->tamItem);

    memcpy(&elremovido, &stack->itens[stack->topo - 1], stack->tamItem);
    stack->topo--;
    return elremovido;
}

void *top(struct Stack *stack)
{
    if (estaVazio(stack))
        return 0;

    void *elTopo = malloc(stack->tamItem);

    memcpy(&elTopo, &stack->itens[stack->topo - 1], stack->tamItem);
    return elTopo;
}

void deleteStack(struct Stack *stack)
{
    while (stack->topo != 0)
    {
        pop(stack);
    }
    free(stack);
}