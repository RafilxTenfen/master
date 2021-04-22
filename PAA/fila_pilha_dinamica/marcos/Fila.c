#include <stdio.h>
#include <stdlib.h>
#include "Fila_privado.h"
#include <string.h>
#include <ctype.h>
#define SUCESSO 0
#define ERRO 1

// Funcao de criar a fila
struct Fila *criar(int tamanho_dados, int *resultado)
{
    fila_t *fila = (fila_t *)malloc(sizeof(fila_t));
    if (fila == NULL)
    {
        return fila;
    }
    fila->tamanho_dados = tamanho_dados;
    fila->inicio = NULL;
    fila->fim = NULL;
    printf("\nA fila foi criada no TDA-Frank\n\n");
    return fila;
}

// Enfileira elementos na fila
void enfileirar(struct Fila *fila, void *elemento, int *resultado)
{
    int *estado = (int *)ERRO;
    resultado = estado;
    if (fila == NULL)
        return;

    no_t *nova_entrada = (no_t *)malloc(sizeof(no_t));
    nova_entrada->dados = malloc(sizeof(fila->tamanho_dados) + 1);
    if (nova_entrada == NULL)
    {
        exit(0);
    }
    nova_entrada->dados = elemento;
    nova_entrada->ant = NULL;
    if (fila->inicio == NULL)
    {
        fila->inicio = fila->fim = nova_entrada;
        nova_entrada->ant = nova_entrada;
        estado = (int *)SUCESSO;
        printf("\nA Pilha %p foi enfileirada\n", nova_entrada->dados);
        return;
    }
    no_t *ultimo_no = fila->fim;
    ultimo_no->ant = nova_entrada;
    fila->fim = nova_entrada;
    estado = (int *)SUCESSO;
    printf("\nA Pilha %p foi enfileirada\n", nova_entrada->dados);
    return;
}

// Desenfileira o primeiro elemento da fila
void *desenfileirar(struct Fila *fila, int *resultado)
{
    resultado = (int *)ERRO;
    if (fila == NULL)
        return NULL;
    if (fila->inicio == NULL)
    {
        return NULL;
    }
    void *elem_removido = malloc(fila->tamanho_dados);
    no_t *antigo_ultimo;
    no_t *unico_no;

    if (fila->fim == fila->inicio)
    {
        unico_no = fila->inicio;
        memcpy(&elem_removido, &unico_no->dados, fila->tamanho_dados);

        free(unico_no);
        fila->inicio = NULL;
        resultado = (int *)SUCESSO;
        printf("\nA Pilha %p foi desenfileirada\n\n", elem_removido);
        return elem_removido;
    }

    memcpy(&elem_removido, &fila->inicio->dados, fila->tamanho_dados);
    antigo_ultimo = fila->inicio;
    fila->inicio = antigo_ultimo->ant;
    resultado = (int *)SUCESSO;
    printf("\nA Pilha %p foi desenfileirada\n\n", elem_removido);
    return elem_removido;
}

// Destroi fila
void destruir_fila(struct Fila *fila, int *resultado)
{
    if (fila == NULL)
        return;

    while (fila->inicio != NULL)
    {
        int *resultado = NULL;
        desenfileirar(fila, resultado);
    }
    //free(fila); A fila será destruída junto ao destroi do TDA Frank
    resultado = (int *)SUCESSO;
    printf("A Fila foi destruida\n");
}

int cheia(struct Fila *fila)
{
    printf("Funcao nao implementada pois a fila e dinamica.\n");
    return ERRO;
}

int vazia(struct Fila *fila)
{
    if (fila->inicio == NULL)
    {
        return SUCESSO;
    }
    return ERRO;
}

// Funcao adicionada: retorna o inicio da fila sem desenfileirar
void *retorna_inicio(struct Fila *fila, int *resultado)
{
    int *estado = (int *)ERRO;
    resultado = estado;

    if (fila == NULL)
        return NULL;

    if (fila->inicio == NULL)
    {
        return NULL;
    }

    void *elem_removido = malloc(fila->tamanho_dados);
    no_t *antigo_ultimo;
    no_t *unico_no;

    if (fila->fim == fila->inicio)
    {
        unico_no = fila->inicio;
        memcpy(&elem_removido, &unico_no->dados, fila->tamanho_dados);

        estado = (int *)SUCESSO;
        return elem_removido;
    }
    antigo_ultimo = fila->inicio;
    memcpy(&elem_removido, &antigo_ultimo->dados, fila->tamanho_dados);
    estado = (int *)SUCESSO;
    resultado = estado;
    return elem_removido;
}

// Funcao adicionada: retorna o fim da fila sem modificar a fila
void *retorna_fim(struct Fila *fila, int *resultado)
{
    int *estado = (int *)ERRO;
    resultado = estado;
    if (fila == NULL)
        return NULL;

    if (fila->inicio == NULL)  // Se a fila esta vazia, retorna NULL
    {
        printf("Fila vazia!\n");
        return NULL;
    }

    void *elem_removido = malloc(fila->tamanho_dados);
    no_t *antigo_ultimo;
    no_t *unico_no;

    // Se a fila somente tem um elemento
    if (fila->fim == fila->inicio)
    {
        unico_no = fila->fim;
        memcpy(&elem_removido, &unico_no->dados, fila->tamanho_dados);
        estado = (int *)SUCESSO;
        return elem_removido;
    }

    // Caso a fila tenha varios elementos, retornamos o que esta contido no ultimo, sem modificar a fila
    antigo_ultimo = fila->fim;
    memcpy(&elem_removido, &antigo_ultimo->dados, fila->tamanho_dados);
    estado = (int *)SUCESSO;
    resultado = estado;
    return elem_removido;
}

// Funcao adicionada: permite modificar o ultimo elemento da fila, sem alterar os demais
void atualiza_ultimo(struct Fila *fila, void *elemento, int *resultado)
{
    int *estado = (int *)ERRO;
    resultado = estado;
    if (fila == NULL)
        return;

    if (fila->inicio == NULL) // Se a fila esta vazia, adiciona esse elemento
    {
        no_t *nova_entrada = (no_t *)malloc(sizeof(no_t));
        nova_entrada->dados = malloc(sizeof(fila->tamanho_dados) + 1);
        if (nova_entrada == NULL)
        {
            exit(0);
        }
        nova_entrada->dados = elemento;
        nova_entrada->ant = NULL;
        fila->inicio = fila->fim = nova_entrada;
        nova_entrada->ant = nova_entrada;
        estado = (int *)SUCESSO;
        return;
    }

    // Se a fila nao esta vazia, modifica o ultimo elemento da fila sem modificar os demais.
    memcpy(&fila->fim->dados, &elemento, fila->tamanho_dados);

    estado = (int *)SUCESSO;
    return;
}