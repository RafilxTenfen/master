#include "Pilha_interface.h"
#include "Fila_publica.h"
#include "Frank_publica.h"

// Define a estrutura da fila
struct Frank {
    int tamanho_dados;      // Tamanho dos dados na fila
    fila_t *fila;               // Elemento fila
};
