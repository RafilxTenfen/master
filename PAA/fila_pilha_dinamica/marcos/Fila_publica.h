#ifndef __FILA_H__
#define __FILA_H__

#include <stdbool.h>

typedef struct Fila fila_t;
typedef struct NoFila no_t;

struct Fila * criar (int tamanho_dados, int * resultado);
void enfileirar (struct Fila * fila, void * elemento, int * resultado);
void * desenfileirar(struct Fila * fila, int * resultado);
void destruir_fila (struct Fila * fila, int * resultado);

// Funcao adicionada: retorna o primeiro elemento da fila sem desenfileirar
void * retorna_inicio(struct Fila * fila, int * resultado);

// Funcao adicionada: retorna o ultimo elemento da fila sem modificá0-la
void * retorna_fim(struct Fila * fila, int * resultado);

// Função adicionada: permite modificar o ultimo elemento da fila sem alterar os demais
void atualiza_ultimo (struct Fila * fila, void * elemento, int * resultado);

#endif /* __FILA_H__ */
