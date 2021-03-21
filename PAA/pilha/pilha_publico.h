/* pilha_publico.h */

typedef struct PE *pPilha, **ppPilha;

// alloca memoria para pilha
int criaPilha(ppPilha pp, int tamanhoVetor, int tamanhoInfo);
// libera a memoria ocupada da pilha
int destroiPilha(ppPilha pp);

// adiciona elemento a pilha
int empilha(pPilha p, void *elemento);
// remove elemento da pilha
int desempilha(pPilha p, void *elemento);

// libera memoria e reinicia a pilha aos valores iniciais
int reiniciaPilha(pPilha p);
// seta o valor do topo da pilha no parametro topo
int topo(pPilha p, void *topo);
