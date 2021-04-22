#include <stdio.h>
#include "Fila_publica.h"

int main(int argc, char** argv) {
  int result;
  pFila f = criar(sizeof(int), &result);
  printf("Criar result: %d", result);

  int enf1 = 4;
  enfileirar(f, &enf1, &result);
  printf("\nenfileirar result: %d -> %d", result, enf1);

  // int wrongResult;
  // enfileirar(f, 3, &wrongResult);
  // printf("\nenfileirar wrong wrongResult: 1 == %d", wrongResult);

  int enf2 = 9;
  enfileirar(f, &enf2, &result);
  printf("\nenfileirar result: %d -> %d", result, enf2);

  int enf3 = 15;
  enfileirar(f, &enf3, &result);
  printf("\nenfileirar result: %d -> %d", result, enf3);

  int enf4 = 65;
  enfileirar(f, &enf4, &result);
  printf("\nenfileirar result: %d -> %d", result, enf4);

  int enf5 = 99;
  enfileirar(f, &enf5, &result);
  printf("\nenfileirar result: %d -> %d", result, enf5);

  int *desenf1;
  desenf1 = desenfileirar(f, &result);
  printf("\ndesenfileirar result: %d -> %d", result, *desenf1);

  int *desenf2;
  desenf2 = desenfileirar(f, &result);
  printf("\ndesenfileirar result: %d -> %d", result, *desenf2);

  int *desenf3;
  desenf3 = desenfileirar(f, &result);
  printf("\ndesenfileirar result: %d -> %d", result, *desenf3);

  destruir(f, &result);
  printf("\ndestruir ints result: %d\n", result);

  printf("\nTest Chars\n\n");
  // Test using char
  int resultc;
  pFila fc = criar(sizeof(char*), &resultc);
  printf("Criar resultc: %d", resultc);

  char* enfc1 = "FIRST";
  enfileirar(fc, enfc1, &resultc);
  printf("\nenfileirar resultc: %d -> %s", resultc, enfc1);

  char* enfc2 = "SECOND";
  enfileirar(fc, enfc2, &resultc);
  printf("\nenfileirar resultc: %d -> %s", resultc, enfc2);

  char* enfc3 = "THIRD";
  enfileirar(fc, enfc3, &resultc);
  printf("\nenfileirar resultc: %d -> %s", resultc, enfc3);


  printf("\n\nDESINFILEIRAR\n");

  char* desenfc1;
  desenfc1 = desenfileirar(fc, &resultc);
  printf("\ndesenfileirar resultc: %d -> %s", resultc, desenfc1);

  // destruir(fc, &resultc);
  // printf("\ndestruir result: %d\n", resultc);

  char* desenfc2;
  desenfc2 = desenfileirar(fc, &resultc);
  printf("\ndesenfileirar resultc: %d -> %s", resultc, desenfc2);

  char* desenfc3;
  desenfc3 = desenfileirar(fc, &resultc);
  printf("\ndesenfileirar resultc: %d -> %s", resultc, desenfc3);

  char* desenfc4;
  desenfc4 = desenfileirar(fc, &resultc);
  printf("\ndesenfileirar fila vazia resultc: %d -> %s", resultc, desenfc4);

  // Test destruir
  int resultf;
  pFila ff = criar(sizeof(char*), &resultf);
  printf("\nCriar resultf: %d", resultf);
  destruir(ff, &resultf);
  printf("\nDestruir resultf: %d", resultf);
  destruir(NULL, &resultf);
}
