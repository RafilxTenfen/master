#include "main.h"

int main(int argc, char* argv[]) {
  typedef char* my_type;
  struct pilha* P = Init();
  struct pilha* PINT = Init();
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
  //   int count = 0;
  //   char buf[1024];
  //   /* assumes no word exceeds length of 1023 */
  //   while (fscanf(fp, "%s", buf) == 1) {
  //     count++;
  //     for (int i = 0; i < 1024; i++) {
  //       char bufI = buf[i];
  //       if (bufI == '\0') {
  //         printf("push int: %d\n", i);
  //         // Push(pInteiros, &i);
  //         break;
  //       }
  //     }
  //     printf("push: %s\n", buf);
  //     Push(P, &buf);
  //   }
  int count = 0;
  while ((read = getline(&line, &len, fp)) != -1) {
    printf("Retrieved line of length %zu:\n", read);
    int size_word = (int)read - 1;
    printf("%d - %s", size_word, line);
    //   if (line[0] == 'a')
    //   {
    strcpy(arg[count], line);
    ret = Push(P, arg[count]);
    Push(PINT, size_word);
    count = count + 1;
    //   }
  }

  fclose(fp);
  printf("\n");

  int countTotal = 0;
  tamanho = Size(P);
  printf("There are %d words in the stack:\n", tamanho);
  for (int i = 0; i < tamanho; i++) {
    char* topoChar = Top(P);
    void* topoInt = Top(PINT);
    ret = Pop(PINT);
    ret = Pop(P);

    if (topoChar[0] == 'z' || topoChar[0] == 'Z') {
      int restOfWords = tamanho - (i + 1);
      printf("\nRest of words in stack %d", restOfWords);

      for (int j = 0; j < restOfWords; j++) {
        int* topoInt2 = Top(PINT);
        int topoIntSPoint = (int)topoInt2;
        Pop(PINT);
        printf("\npop topoInt2: %d", topoInt2);
        countTotal += topoIntSPoint;
      }
      printf("\nCount of chars in the rest of stack %d\n", countTotal);
      break;
    }

    if (ret == 0)
      printf("Elemento desempilhado (topo %d - '%s' removido)\n", ((int)topoInt), topoChar);
    else
      printf("Erro no desempilhamento\n");
  }

  Destroy(PINT);
  Destroy(P);
  printf("Pilha destruida\n");
}
