typedef struct FilaPilha FilaPilha;

int filaPilhaCriar(FilaPilha **fila_pilha, int tamanho_pilha, int tamanho_dados);
int filaPilhaDestruir(FilaPilha **fila_pilha);

int filaPilhaInserir(FilaPilha *fila_pilha, void *dados);
int filaPilhaRemover(FilaPilha *fila_pilha, void *dados);

int filaPilhaVazia(FilaPilha *fila_pilha);