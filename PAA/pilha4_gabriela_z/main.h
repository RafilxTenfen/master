#include "pilha_interface.h"

#define BUF_MAX_SIZE 1024 // max size of a word in the file

// callback for read each word and push into the stack
typedef void (*callbackReadWord)(char*, pPilha);

// callback for read each word and push into the stack
typedef void (*callbackReadCount)(int, pPilha);

// read all the words that starts with 'a' from the file
int readWordsFromFile(FILE* f, pPilha pilhaWords, pPilha pilhaWordWidth, callbackReadWord callBackWord, callbackReadCount callBackCount);

// read one word at time
void readWord(char* word, pPilha p);

// read the word and count the size of the word
void readCount(int widthWord, pPilha p);

// verify if char is the char 'z' and 'Z', 1 is true, 0 is false
int charIsZ(char c);