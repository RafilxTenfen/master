
#include <stdio.h>
#include <stdlib.h>
#include "pilhaPublico.h"

int main(int argc, char **argv) {
  pilha* p1;
  removerPilha(p1);
  pilha* p = criarPilha(4, sizeof(int));

  pop(p);
  push(p, 4);
  push(p, 2);
  removerPilha(p);
  pop(p);
  pop(p);
  pop(p);
  push(p, 8);
  mostrarPilha(p);

  consultarTopo(p);
}