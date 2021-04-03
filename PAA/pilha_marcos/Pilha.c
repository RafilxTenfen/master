#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "Pilha_privado.h"

const int STACK_DATA_SIZE = 8;

int cheia(Pilha* stack) {
	return stack->topo == stack->stackSize - 1;
} ;

int vazia(Pilha* stack) {
	return stack->topo == -1;
} ;

Pilha* inicia_pilha(int stackSize) {
	Pilha *stack = malloc(sizeof(Pilha));
	stack->topo = -1;
	stack->stackSize = stackSize;
	stack->stackData = STACK_DATA_SIZE;
	stack->data = malloc((stackSize * STACK_DATA_SIZE) * sizeof(void*));
	return stack;
} ;

void destroi_pilha(Pilha** stack) {
	int i = 0;
	for (i ; i < (*stack)->topo; i++) {
		(*stack)->data[i] = NULL;
		free((*stack)->data[i]);
	}
	(*stack) = NULL;
	free((*stack));
	printf("A pilha foi destruida\n");
	return;
} ;

void empilha(Pilha* stack, void* elemento) {
	if(stack == NULL) {
		printf("A pilha nao foi inicializada\n");
		return;
	}
	if(cheia(stack)) {
		printf("A pilha esta cheia\n");
		return;
	}
	stack->topo++;
	stack->data[stack->topo] = elemento;
	printf("O Elemento com endereco %p foi empilhado\n\n", &stack->data[stack->topo]);
	return;
} ;

void* desempilha(Pilha* stack) {
	if(stack == NULL) {
		printf("A pilha nao foi inicializada\n");
		return NULL;
	}
	if(vazia(stack)) {
		printf("A pilha esta vazia\n");
		return NULL;
	}
	void* removido;
	memcpy(&removido, &stack->data[stack->topo], stack->stackData);
	stack->data[stack->topo] = NULL;
	stack->topo--;
	printf("O Elemento com endereco %p foi desempilhado\n\n", &stack->data[stack->topo]);
	return removido;
} ;

void* pega_topo(Pilha* stack) {
	if(stack == NULL) {
		printf("A pilha nao foi inicializada\n");
		return NULL;
	}
	if(vazia(stack)) {
		printf("A pilha esta vazia\n");
		return NULL;
	}
	void* topo;
	memcpy(&topo, &stack->data[stack->topo], stack->stackData);
	printf("Endereco de memoria do topo: %p\n", &stack->data[stack->topo]);
	return topo;
} ;

/*
DICA:
	Esta fun��o � gen�rica mas est� setada para imprimir strings(%s). Para imprimir outros tipos � necess�rio alterar
	para o tipo de dado espec�fico %d - %f - %c, etc, ou capturar o retorno no main e fazer o casting.
*/
void* mostra_pilha(Pilha* stack) {

	if(stack == NULL) {
		printf("A pilha nao foi inicializada\n");
		return NULL;
	}
	if(vazia(stack)) {
		printf("A pilha esta vazia\n");
		return NULL;
	}
	char* i; //mudar para void se for retornar para a main
	int numElement = stack->topo;
	printf("\nExibindo o conteudo da pilha:\n");
	int aux = 0;
	for (aux = numElement; aux > -1; aux--) {
		i = (stack->data[aux]);
		printf("\n%s", i);
	}
	return i;
} ;
