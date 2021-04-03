#ifndef _fila_publica
#define _fila_publica

#define TRUE 0
#define FALSE 1

typedef struct Fila *pFila;

pFila criar(int tamanho_dados, int *resultado);
void enfileirar(pFila f, void *elemento, int *resultado);
void* desenfileirar(pFila f, int *resultado);
void destruir(pFila f, int *resultado);

#endif