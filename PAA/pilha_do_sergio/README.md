## Pilha dinamica em C

### Dependencias
- [linux](https://www.linux.org/pages/download/)
- [GCC](https://gcc.gnu.org/install/index.html)
- [make](https://www.unixmen.com/install-ubuntu-make-on-ubuntu-15-04/)

### Métodos Publicos
```c
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

--> o arquivo de palavras está em words.txt


### Como executar?
- A aplicação está no arquivo main.c
```shell
$~ make clean
```
- Execute o comando a seguir para gerar o binário
```shell
$~ make
```
- Caso o comando make tenha funcionado corretamente
```shell
$~ ./aplicacao
```