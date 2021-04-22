#include <stdio.h>
#include <stdlib.h>
#include "Frank_publica.h"

int main(int argc, char **argv)
{
    int *resultado;
    void *dado = malloc(sizeof(int *));

    //Criando o TDA-Frank
    pFrank *f = cria(sizeof(int), resultado);


    // Prepara os elementos para sequenciamento
    int *a;
    a = (int *)100;
    int *b;
    b = (int *)80;
    int *c;
    c = (int *)2;
    int *d;
    d = (int *)4;
    int *e;
    e = (int *)10;
    int *ff;
    ff = (int *)3;
    int *g;
    g = (int *)99;
    int *h;
    h = (int *)84;
    int *i;
    i = (int *)222;


    // Inicia o sequenciamento dos elementos no Frank
    printf("\nInserindo na primeira pilha\n");

    insere(f, a, resultado);  // Usa a função insere para adicionar elementos ao Frank
    insere(f, b, resultado); 
    insere(f, c, resultado); 
    printf("\n");

    printf("\nInserindo na segunda pilha\n");

    insere(f, d, resultado); 
    insere(f, e, resultado); 
    insere(f, ff, resultado);
    printf("\n"); 
    

    printf("\nInserindo na terceira pilha\n");

    insere(f, g, resultado); 
    insere(f, h, resultado);
    insere(f, i, resultado); 
    printf("\n");



    // Inicia a remoção dos elementos do Frank
    printf("\nRemovendo os elementos da primeira pilha\n");

    dado = remover(f, resultado);  // Usa a função remove para remover elementos do Frank, um a um, seguindo a ordem da fila/pilha
    printf("O Elemento %d foi desenfileirado do frank\n", (int *)dado);

    dado = remover(f, resultado);
    printf("O Elemento %d foi desenfileirado do frank\n", (int *)dado);

    dado = remover(f, resultado);
    printf("O Elemento %d foi desenfileirado do frank\n", (int *)dado);

    printf("\nRemovendo os elementos da segunda pilha\n");

    dado = remover(f, resultado);
    printf("O Elemento %d foi desenfileirado do frank\n", (int *)dado);

    dado = remover(f, resultado);
    printf("O Elemento %d foi desenfileirado do frank\n", (int *)dado);

    dado = remover(f, resultado);
    printf("O Elemento %d foi desenfileirado do frank\n", (int *)dado);


    printf("\nRemovendo os elementos da terceira pilha\n");

    dado = remover(f, resultado);
    printf("O Elemento %d foi desenfileirado do frank\n", (int *)dado);

    dado = remover(f, resultado);
    printf("O Elemento %d foi desenfileirado do frank\n", (int *)dado);

    dado = remover(f, resultado);
    printf("O Elemento %d foi desenfileirado do frank\n", (int *)dado);



    //Destruindo o TDA-Frank
    destruir(f, resultado);

}
