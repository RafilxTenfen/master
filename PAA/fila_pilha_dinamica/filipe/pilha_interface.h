typedef struct Pilha Pilha;

int pilhaCriar(Pilha **pilha, int tamanho, int tamanho_dados);
int pilhaDestruir(Pilha **pilha);

int pilhaEmpilhar(Pilha *pilha, void *dados);
int pilhaDesempilhar(Pilha *pilha, void *dados);

int pilhaTopo(Pilha *pilha, void *dado);
int pilhaCheia(Pilha *pilha);
int pilhaVazia(Pilha *pilha);