#include "Pilha_interface.h"

int cheia(Pilha* stack);
int vazia(Pilha* stack);

struct Stack {
	void** data;
	int topo;
	int stackSize;
	int stackData; 
};


