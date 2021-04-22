#define SUCESSO 0
#define FRACASSO 1

typedef struct Frank pFrank;

pFrank* cria(int tamanho_dados, int *resultado);

void insere(pFrank *F, void *dado, int *resultado);

void *remover(pFrank *F, int *resultado);

void destruir(pFrank *F, int *resultado); 