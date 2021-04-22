typedef struct Fila Fila;

int filaCriar(Fila **fila, int tamanho_dados);
int filaDestruir(Fila **fila);

int filaEnfileirar(Fila *fila, void *dados);
int filaDesenfileirar(Fila *fila, void *dados);

int filaCauda(Fila *fila, void *dados);
int filaCabeca(Fila *fila, void *dados);
int filaTamanho(Fila *fila);
