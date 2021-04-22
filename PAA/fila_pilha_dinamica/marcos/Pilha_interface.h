/* pilha_publico.h */

typedef struct Stack *pPilha, **ppPilha;

//Cria uma pilha, com o tamanho por parametro
struct Stack* createStack(int tamItem);

// Função que empilha um item na pilha
void push(struct Stack* stack, void *item);

// Função que desempilha um item da pilha
void *pop(struct Stack* stack);

// Funcao que retorna o topo da pilha, sem remove-lo.
void *top(struct Stack* stack);

// Funcao que deleta toda a pilha (internamente ela verifica a pilha e enquanto existir elemento, ela desempilha)
void deleteStack(struct Stack *stack);

// Funcao que indica se a pilha esta cheia
int estaCheio(struct Stack* stack);

// Funcao que indica se a pilha esta vazia
int estaVazio(struct Stack* stack);

#define PILHA_TAM_MAX 3