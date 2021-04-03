#include "main.h"

int main(int argc, char* argv[]) {
  typedef char* my_type;
  struct pilha* P = Init(sizeof(int));
  struct pilha* pInteiros = Init(sizeof(char));
  if (P == NULL)
    printf("Falha na criacao da pilha\n");
  else
    printf("Pilha criada\n");

  int ret = -1;
  void* topo = 0;
  int tamanho = 0;
  char arg[500][100];

  FILE* fp;
  char* line = NULL;
  size_t len = 0;
  size_t read;

  fp = fopen("./lista_palavras.txt", "r");
  if (fp == NULL) {
    printf("Could not open file.\n");
    exit(EXIT_FAILURE);
  }

  int count = 0;
  char buf[1024];
  /* assumes no word exceeds length of 1023 */
  while (fscanf(fp, "%s", buf) == 1) {
    count++;
    for (int i = 0; i < 1024; i++) {
      char bufI = buf[i];
      if (bufI == '\0') {
        printf("push int: %d\n", i);
        Push(pInteiros, &i);
        break;
      }
    }
    printf("push: %s\n", buf);
    Push(P, buf);
  }

  // while ((read = getline(&line, &len, fp)) != -1) {
  //   printf("Retrieved line of length %zu:\n", read);
  //   printf("%s - %zu\n", line, len);
  //   // if (line[0] == 'a') {
  //   strcpy(arg[count], line);
  //   ret = Push(P, arg[count]);
  //   Push(pInteiros, len);
  //   count = count + 1;
  //   // }
  // }

  fclose(fp);
  printf("\n");

  tamanho = Size(P);
  printf("There are %d words in the stack:\n", tamanho);
  for (int i = 0; i < tamanho; i++) {
    int* iPop = Top(pInteiros);
    printf("pop: %d\n", *iPop);
    char* cPop = Top(P);
    printf("pop: %s\n", cPop);

    Pop(P);
    Pop(pInteiros);
    // if (ret == 0)
    //   printf("Elemento desempilhado (topo '%s' removido)\n", cPop);
    // else
    //   printf("Erro no desempilhamento\n");
  }

  Destroy(P);
  printf("Pilha destruida\n");
}
