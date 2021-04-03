#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "Pilha_interface.h"
#define NOME_ARQUIVO "palavras_aleatorias.txt";  //Alterar conforme a necessidade

int main() {
  int stackSize = 900;
  // int i, j,
	int totalPalavras = 0;
  char* topoDaPilha;

  FILE* arq;
  char* arquivo = NOME_ARQUIVO;
  // int memQtd = 300;
  // char vetorInicial[300];
  // char **vetorTemporario = malloc(memQtd* sizeof(char*));
  // char **vetPalavras = malloc(memQtd* sizeof(char*));

  printf("Defina a capacidade total da pilha: ");
  // scanf("%d", &stackSize);

  //Criando/Inicializando a pilha
  Pilha* p = inicia_pilha(stackSize);
  if (p == NULL) {
    printf("Ocorreu um erro ao criar a pilha\n");
  } else {
    printf("A pilha foi criada com sucesso\n\n");
  }

  arq = fopen(arquivo, "r");
  if (arq == NULL) {
    printf("Erro ao abrir o arquivo: %s\n", arquivo);
    return 1;
  }

  // printf("===================================\n");
  // printf("Lendo elementos e empilhando...\n\n");

  char buf[1024];
  while (fscanf(arq, "%s", buf) == 1) {
    char buf0 = buf[0];
    printf("\npush: %s", buf);

    if (buf0 == 'a' || buf0 == 'A') {
			totalPalavras++;
      empilha(p, &buf0);
    }
  }

  //   while (fgets(vetorInicial, sizeof(vetorInicial), arq) != NULL) {
  // 	vetorTemporario[i] = strdup(vetorInicial);
  // 	memcpy(vetorInicial, vetorTemporario[i], strlen(vetorTemporario[i]));

  // 	printf("palavra lida: %s\n", vetorInicial);
  // 	if (vetorInicial[0] == 'a' || vetorInicial[0] == 'A') {
  // 		vetPalavras[i] = strdup(vetorInicial);
  // 		printf("Palavra encontrada: %s", vetPalavras[i]);

  // 		//Chamando fun��o de empilhar
  // 		empilha(p, vetPalavras[i]);
  // 		i++;
  // 		totalPalavras++;
  // 	}
  // }

  // char* cTop = top(p);
  // printf("\nTopo capturado: %s \n", cTop);
  // printf("Numero total de palavras empilhadas: %d\n", totalPalavras);
  // printf("===================================\n");

  // //Exibindo a pilha
  // mostra_pilha(p);
  // printf("===================================\n\n");

  // printf("Desempilhando os elementos\n\n");
  for (int j = 0 ; j < totalPalavras ; j++) {
  	//Chamando fun��o de desempilhar
	  char* cDesempilha = desempilha(p);
  	printf("\nDesempilha: %s \n", &cDesempilha);
  }
  // printf("===================================\n");

  fclose(arq);
  // free(vetorTemporario);
  // free(vetPalavras);
  destroi_pilha(&p);

  return 0;
}
