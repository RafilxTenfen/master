#include "main.h"

#include <stdio.h>
#include <stdlib.h>

#include "pilha_publico.h"

int main(int argc, char** argv) {
  int i = 0;
  int numPalavras = 0;
  //Programa assume que as palavras tem até 50 letras
  char* palavras[50];
  char line[50];

  FILE* arquivo;
  arquivo = fopen("words.txt", "r");

  while (fgets(line, sizeof line, arquivo) != NULL) {
    palavras[i] = strdup(line);
    i++;
    numPalavras++;
  }

  i = 0;
  int j;

  pPilha p;
  pPilha pInt;
  int nElementos = numPalavras;
  printf("nElementos %d\n", nElementos);
  criaPilha(&p, nElementos, sizeof(char*));
  criaPilha(&pInt, 60, sizeof(int));

  for (j = 0; j < numPalavras; j++) {
    char* word = palavras[j];
    printf("\nEmpilhando %s", palavras[j]);
    empilha(p, word);

    for (int k = 0; k < 50; k++) {
      char bufI = word[k];
      printf("k: %s - %d ", &bufI, k);
      if (bufI == '\0') {
        int countWord = k - 1;

        printf("Empilhando %d - k: %d\n", countWord, k);
        empilha(pInt, &countWord);
        break;
      }
    }
  }

  printf("\n\n");

  fclose(arquivo);

  void* topoChar;
  topo(p, &topoChar);
  printf("\n topo is %s", (char*)&topoChar);

  printf("\n\n");

  int k;

  for (k = 0; k < numPalavras; k++) {
    char* word = palavras[k];
    printf("\nDesempilhando %s", palavras[k]);
    desempilha(p, word);
  }

  return EXIT_SUCCESS;
}