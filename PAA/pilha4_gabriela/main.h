#include "pilha_interface.h"

#define BUF_MAX_SIZE 1024 // max size of a word in the file

// callback for read each word and push into the stack
typedef void (*callbackReadWord)(char*, pPilha);

// read all the words that starts with 'a' from the file
int read_words_start_a(FILE* f, pPilha p, callbackReadWord callBack);

// read one word at time
void readJustA(char* word, pPilha p);

// verify if char is 'a' or 'A', 1 is true, 0 is false
int containA(char buf);