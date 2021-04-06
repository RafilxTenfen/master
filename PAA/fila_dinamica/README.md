## Fila dinamica em C

### Dependencias
- [linux](https://www.linux.org/pages/download/)
- [GCC](https://gcc.gnu.org/install/index.html)
- [make](https://www.unixmen.com/install-ubuntu-make-on-ubuntu-15-04/)

### Métodos Publicos
```c
typedef struct Fila *pFila;

// cria um fila e retorna um ponteiro para a fila
pFila criar(int tamanho_dados, int *resultado);
// enfileira um novo elemento ao fim da fila
void enfileirar(pFila f, void *elemento, int *resultado);
// desinfileira o primeiro elemento da fila
void* desenfileirar(pFila f, int *resultado);
// destroi a fila liberando memoria
void destruir(pFila f, int *resultado);
```

### Como executar?
- Crie o seu arquivo `main.c` com o metodo main
- Importe o arquivo publico
```c
/* main.c */
#include "pilha_publico.h"
int main(int argc, char **argv) {
  ...
  int result;
  pFila f = criar(sizeof(int), &result);

  destruir(f, &result);

  int resultc;
  pFila fc = criar(sizeof(char*), &resultc);

  char* enfc1 = "FIRST";
  enfileirar(fc, enfc1, &resultc);

  char* enfc2 = "SECOND";
  enfileirar(fc, enfc2, &resultc);

  char* desenfc1;
  desenfc1 = desenfileirar(fc, &resultc);

  char* desenfc2;
  desenfc2 = desenfileirar(fc, &resultc);

  destruir(fc, &resultc);
}
```

- Execute o comando a seguir para gerar o binário
```shell
$~ make
```

- Caso o comando make tenha funcionado corretamente
```shell
$~ ./main
```