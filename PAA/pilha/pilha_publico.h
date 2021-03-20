/* pilha_publico.h */

typedef struct PE *pPilha, **ppPilha;

int criaPilha(ppPilha pp, int tamanhoVetor, int tamanhoInfo);
int destroiPilha(ppPilha pp);

int empilha(pPilha p, void *elemento);
int desempilha(pPilha p, void *elemento);

int reiniciaPilha(pPilha p);
int topo(pPilha p, void *topo);
