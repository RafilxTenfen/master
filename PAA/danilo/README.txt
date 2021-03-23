Dependencias:
    Compilador GCC

Métodos Publicos

// Insere um valor na pilha
void push(int x);

// Remove o último valor da pilha
int pop(void);

// Mostra os elementos na pilha
void display(void);

// Mostra o último elemento incluído (o que está no topo da pilha).
int top(void);

Como executar?

    Crie o seu arquivo aplicacao.c com o metodo main
    Importe o arquivo publico

/* aplicacao.c */
#include "Stack_Public.h"
int main(int argc, char **argv) {
  ...
  push(...);
  top();
  push(...);
  top();
  push(...);
  top();
  display();
  pop();
  display();
  top();

  return 0;
}

    Execute o comando a seguir para gerar o binário

$~ make

    Caso o comando make tenha funcionado corretamente

$~ ./aplicacao