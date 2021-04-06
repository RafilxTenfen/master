## Pilha dinamica em C

### Dependencias
- [linux](https://www.linux.org/pages/download/)
- [GCC](https://gcc.gnu.org/install/index.html)
- [make](https://www.unixmen.com/install-ubuntu-make-on-ubuntu-15-04/)

### Métodos Publicos
```c
#define SUCCESSO 0
#define ERRO 1

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
```


### Como executar?
- Crie o seu arquivo `aplicacao.c` com o metodo main
- Importe o arquivo publico
```c
/* aplicacao.c */
#include "pilha_publico.h"
int main(int argc, char **argv) {
  ...
  criaPilha(...);

  empilha(...);
  desempilha(...);
  reiniciaPilha(...);

  empilha(...);
  topo(...);
  destroiPilha(...);
}
```

- Execute o comando a seguir para gerar o binário
```shell
$~ make
```
- Caso o comando make tenha funcionado corretamente
```shell
$~ ./aplicacao
```