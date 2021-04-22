/* pilha_privado.h */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "Pilha_interface.h"

struct Stack {
    int topo;
    int tamItem;
    void *itens[PILHA_TAM_MAX];
};
