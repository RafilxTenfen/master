#include <stdio.h>
#include <stdlib.h>
#include "pilhaPrivado.h"


pilha* criarPilha(int capacidadePilha, int tamanhoDados)
{
   
    pilha* p = (pilha*)malloc(capacidadePilha);
    p->topo = -1;
    p->tamanho = capacidadePilha;
    p->tamanhoDados = tamanhoDados;
    p->dados = (void*) malloc((capacidadePilha*tamanhoDados) * sizeof(void*));   

    if(p == NULL){
	printf("Erro na alocação da pilha\n");
	return NULL;	
    } else {
        if (p->dados == NULL) {
		printf("Erro na alocação dos dados\n");
		return NULL;
        } else {
	    	printf("\nPilha inicializada\n");  
	    	return p;
        }
    }
}

int isEmpty(pilha* p)
{

    if (p->topo == -1)
        return 1;
    else
        return 0;
}

int isFull(pilha* p)
{

    if (p->topo == p->tamanho - 1)
        return 1;
    else
        return 0;
}

void push(pilha* p, int valor) 
{
    if (isFull(p) == 1)
        printf("\nPILHA CHEIA! \n");
    else {

	int* valorAlocado = (int*)malloc(p->tamanhoDados);
        *valorAlocado = valor;
        
        if(valorAlocado == NULL){
		printf("Erro na alocação do valor do elemento\n");		
        } else { 

        	p->topo++;               
        	p->dados[p->topo] = valorAlocado;         

        	int* elementoEmp = (p->dados[p->topo]);
        	printf("Empilhado: %d \n", *elementoEmp);   
	}    
    }
}

void pop(pilha* p)
{

    if (isEmpty(p) == 1)
        printf("\nPILHA VAZIA! \n");
    else {

        int* elementoRemovido = (p->dados[p->topo]);
        printf("Removido: %d \n", *elementoRemovido);
        free(elementoRemovido);
        p->topo--;
    }
}

void consultarTopo(pilha* p)
{

    if (isEmpty(p) == 1) {
        printf("\nPILHA VAZIA!\n");
    }
    else {
        int* topo = *(&p->dados[p->topo]);
        printf("\nTopo: %d \n", *topo);
    }
}

void mostrarPilha(pilha* p)
{

    int* aux;
    int qtdeElementos;
    if (isEmpty(p) == 1) {
        printf("\nPILHA VAZIA!\n");
    }
    else {
        qtdeElementos = p->topo;
       
        printf("\nPILHA: \n");
        for (int cont = qtdeElementos; cont > -1; cont--) {            
            aux = (p->dados[cont]);
            printf("%d \n", *aux);
        }
    }
}

int removerPilha(pilha* p)
{
    if (isEmpty(p) == 1) {
        printf("\nPILHA VAZIA! \n");
        return 0;
    }
    else {
        printf("\nPilha removida!\n");
        free(p);
        return 1;
    }
}

