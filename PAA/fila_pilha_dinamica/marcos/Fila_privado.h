#ifndef __FILA_PRIVADO_H__
#define __FILA_PRIVADO_H__

#include <stdbool.h>
#include "Fila_publica.h"

struct NoFila
{
    void * dados;
    struct NoFila *ant;
};

struct Fila
{
    struct NoFila * inicio, * fim;
    int tamanho_dados;
};

int cheia (struct Fila * fila);
int vazia (struct Fila * fila);

#endif /* __FILA_H__ */

