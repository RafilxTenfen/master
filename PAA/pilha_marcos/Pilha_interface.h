typedef struct Stack Pilha;

Pilha* inicia_pilha(int tamanho_pilha);

void destroi_pilha(Pilha** stack);

void empilha(Pilha* stack, void* elemento);

void* desempilha(Pilha* stack);

void* mostra_pilha(Pilha* stack);

void* pega_topo(Pilha* stack);



