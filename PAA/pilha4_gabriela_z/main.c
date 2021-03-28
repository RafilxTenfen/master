#include "main.h"
#include <stdio.h>
#include <stdlib.h>

int readWordsFromFile(FILE* f, pPilha pilhaWords, pPilha pilhaWordWidth, callbackReadWord callBackWord, callbackReadCount callBackCount) {
  int count = 0;
  char buf[BUF_MAX_SIZE];
  /* assumes no word exceeds length of 1023 */
  while (fscanf(f, "%s", buf) == 1) {
    count++;
    for (int i = 0; i < BUF_MAX_SIZE; i++) {
      char bufI = buf[i];
      if (bufI == '\0') {
        callBackCount(i, pilhaWordWidth);
        break;
      }
    }

    callBackWord(buf, pilhaWords);
  }
  return count;
}

int charIsZ(char c) {
  if (c == 'z' || c == 'Z') {
    return 1;
  }
  return 0;
}

void readWord(char* word, pPilha pilhaWords) {
  printf("\npush: %s", word);
  push(pilhaWords, word);
}

void readCount(int widthWord, pPilha pilhaWordWidth) {
  printf("\nsize of word: %d", widthWord);
  push(pilhaWordWidth, &widthWord);
}

int main(int argc, char** argv) {
  ppPilha ppWords = init(sizeof(char*));
  ppPilha ppWordsWidth = init(sizeof(int));

  printf("Every word will be pushed.");

  FILE* f = fopen("palavras.txt", "r");
  int countWords = readWordsFromFile(f, *ppWords, *ppWordsWidth, readWord, readCount);
  printf("\nquantity of words in the file %d", countWords);

  char* cTop = top(*ppWords);
  printf("\ntop: %s", cTop);

  int* iTop = top(*ppWordsWidth);
  printf("\ntop: %d", *iTop);

  printf("\n------ for of pop %d -----\n", countWords);
  for (int i = 0; i < countWords; i++) {
    char* cPop = pop(*ppWords);
    int* iPop = pop(*ppWordsWidth);
    printf("\npop: %s", cPop);
    printf("\npop: %d", *iPop);

    if (charIsZ(cPop[0])) {
      printf("\nzpop starts with 'z': %s", cPop);
      int countCharsInStack = 0;
      int restOfWords = countWords - (i + 1);
      printf("\nRest of words in stack %d", restOfWords);

      for (int j = 0; j < restOfWords; j++) {
        int* jPop = pop(*ppWordsWidth);
        printf("\npop: %d", *jPop);
        countCharsInStack += *jPop;
      }

      printf("\nCount of chars in the rest of stack %d", countCharsInStack);
      break;
    }
  }

  destroy(ppWords);
  destroy(ppWordsWidth);
}
