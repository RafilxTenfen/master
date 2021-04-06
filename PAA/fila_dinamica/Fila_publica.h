#ifndef _fila_publica
#define _fila_publica

#define SUCCESSO 0
#define ERRO 1

typedef struct Fila *pFila;

// cria um fila e retorna um ponteiro para a fila
pFila criar(int tamanho_dados, int *resultado);
// enfileira um novo elemento ao fim da fila
void enfileirar(pFila f, void *elemento, int *resultado);
// desinfileira o primeiro elemento da fila
void* desenfileirar(pFila f, int *resultado);
// destroi a fila liberando memoria
void destruir(pFila f, int *resultado);

#endif