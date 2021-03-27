

#include <stdio.h>
#include <stdlib.h>

#include "main.h"

int read_words_start_a(FILE* f, pPilha p, callbackReadWord callBack) {
  int count = 0;
  char buf[BUF_MAX_SIZE];
  /* assumes no word exceeds length of 1023 */
  while (fscanf(f, "%s", buf) == 1) {
    char buf0 = buf[0];
    if (containA(buf0) == 1) {
      count++;
      callBack(buf, p);
    }
  }
  return count;
}

int containA(char buf) {
  if (buf == 'a' || buf == 'A') {
    return 1;
  }
  return 0;
}

void readJustA(char* word, pPilha p) {
  printf("\npush: %s", word);
  push(p, word);
}

int main(int argc, char** argv) {
  ppPilha pp = init(sizeof(char*));

  printf("Every word that starts with letter 'a' will be pushed.");

  FILE* f = fopen("palavras.txt", "r");
  int countWordsWithA = read_words_start_a(f, *pp, readJustA);
  printf("\nquantity of words that starts with 'a' %d", countWordsWithA);

  char* cTop = top(*pp);
  printf("\ntop: %s", cTop);

  printf("\nIt will start to pop %d words in the stack", countWordsWithA);
  for (int i = 0; i < countWordsWithA; i++) {
    char* cPop = pop(*pp);
    printf("\npop: %s", cPop);
  }

  destroy(pp);
}
