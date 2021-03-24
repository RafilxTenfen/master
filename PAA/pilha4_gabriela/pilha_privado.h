#include <string.h>
#include "pilha_interface.h"

typedef struct stack_t {
  void *data;
  int head;
  int data_size;
} stack_t;

void move_head_up(stack_t *stack);
void move_head_down(stack_t *stack);
