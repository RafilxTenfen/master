#ifndef __PILHA_PRIVADO_H__
#define __PILHA_PRIVADO_H__

#include <stdbool.h>

#include "Pilha_interface.h"

struct pilha {
	int topo;
	void** elementos;
	int tamanho;
	int sizeOfItem;
};

#endif /* __PILHA_H__ */
