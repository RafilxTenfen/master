#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "Frank_privado.h"

// Funcao para criar o Frank
pFrank *cria(int tamanho_dados, int *resultado)
{
    pFrank *F = (pFrank *)malloc(sizeof(pFrank));
    if (F == NULL)
    {
        printf("\n Falha na alocacao de memoria do Frank\n");
        resultado = (int *)FRACASSO;
        return F;
    }
    else
    {
        printf("\nO TDA-Frank foi criado\n\n");
        F->tamanho_dados = tamanho_dados;
        resultado = (int *)SUCESSO;
        
        // O tamanho dos dados na fila: tamanho do tipo de dado (inteiro, char, etc) * numero de elementos em cada pilha + tamanho da estrutura PILHA
        int tamanho_dados_no_fila = F->tamanho_dados * PILHA_TAM_MAX + sizeof(pPilha);

        //Criando a Fila
        F->fila = criar(tamanho_dados_no_fila, resultado);
        return F;
    }
}

void destruir(pFrank *F, int *resultado)
{
    // Chama a função que remove todos os elementos e destroi fila
    destruir_fila(F->fila, resultado);

    // Libera a memoria do Frank
    free(F);
    printf("O TDA-Frank foi destruido\n\n");
}

// Funcao para inserir elementos no Frank
void insere(pFrank *F, void *dado, int *resultado)
{
    pPilha pilha;
    if (vazia(F->fila) == SUCESSO) // No criar, alocamos a memoria para uma fila, mas ela esta vazia.
    {
        pilha = createStack(F->tamanho_dados);
        push(pilha, dado);
        enfileirar(F->fila, pilha, resultado);
        return; // Termina a funcao insere
    }

    // Se a fila nao esta vazia, buscamos a pilha que esta no fim da fila
    pilha = retorna_fim(F->fila, resultado);
    if (estaCheio(pilha))  // Se esta pilha esta cheia, é necessário adicionar outra
    {
        pilha = NULL;
        pilha = createStack(F->tamanho_dados);
        push(pilha, dado);
        enfileirar(F->fila, pilha, resultado);  // Enfileira a nova pilha na fila
        return;  // Termina a funcao insere
    }
    else  // Se a pilha não está cheia, adiciona o elemento à pilha
    {
        push(pilha, dado);
        atualiza_ultimo(F->fila, pilha, resultado);  // Função adicionada a Fila que permite modificar o ultimo elemento da fila sem a necessidade de desemfileirar toda a fila para modificá-lo
        return;
    }
}

// Funcao para remover elementos do Frank
void *remover(pFrank *F, int *resultado)
{
    pPilha pilha;
    void *dado = malloc(F->tamanho_dados);
    resultado = (int *)FRACASSO;
    if (vazia(F->fila) == SUCESSO)  // Se a fila esta vazia, retorna NULL
    {
        resultado = (int *)FRACASSO;
        return NULL;
    }
    else
    {
        // Busca a pilha que esta no comeco da fila
        pilha = retorna_inicio(F->fila, resultado);
        if (estaVazio(pilha)) // Se e uma pilha vazia (do ultimo remover, por exemplo), desenfileira e pega o proximo
        {
            pilha = NULL;
            pilha = desenfileirar(F->fila, resultado);  // Desenfileira a pilha vazia
            pilha = NULL;
            if (vazia(F->fila) == SUCESSO)  // Se a fila esta vazia, aborta e retorna NULL.
            {
                resultado = (int *)FRACASSO;
                return NULL;
            }
            pilha = retorna_inicio(F->fila, resultado);  // Busca a pilha que esta no começo da fila
            dado = pop(pilha);
            return dado;
        }
        else // Se a pilha nao esta vazia, retorna o elemento do topo da pilha
        {
            dado = pop(pilha);
            return dado;
        }
    }
    return (int *)FRACASSO;
}
