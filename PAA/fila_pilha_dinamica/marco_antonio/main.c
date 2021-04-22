#include "main.h"

int main() {
  FilaPilha *fila_pilha;
  filaPilhaCriar(&fila_pilha, 3, sizeof(int));
  
  int num;
  printf("Digite um número (digite 0 para finalizar): ");
  scanf("%d", &num);

  while (num != 0) {
    filaPilhaInserir(fila_pilha, &num);

    printf("Digite um número (digite 0 para finalizar): ");
    scanf("%d", &num);
  }

  printf("\nNúmeros inseridos:\n");
  while (!filaPilhaVazia(fila_pilha)) {
    filaPilhaRemover(fila_pilha, &num);
    printf("%d\n", num);
  }

  filaPilhaDestruir(&fila_pilha);

  return 0;
}