#include "Stack_Public.h"
#define MAXSIZE 5

struct stack
{
    int stk[MAXSIZE];
    int top;
};
typedef struct stack STACK;
STACK s;
