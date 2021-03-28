#include "pilha_interface.h"
#include "pilha_privado.h"

#define SIZE 100

/* PRIVATE FUNCTIONS */

void move_head_up(pPilha stack) { (*stack).head++; }
void move_head_down(pPilha stack) { (*stack).head--; }

/* PUBLIC FUNCTIONS */

int is_full(pPilha stack) { return stack->head >= SIZE; }
int is_empty(pPilha stack) { return stack->head < 0; }

ppPilha init(int data_size) {
  ppPilha pp = malloc(sizeof(pPilha));
  (*pp) = malloc(sizeof(struct stack_t));
  (*pp)->data_size = data_size;
  (*pp)->head = -1;
  (*pp)->data = malloc(SIZE * data_size);

  if ((*pp)->data == NULL) {
    printf("Error on stack data alloc");
    exit(0);
  }
  return pp;
}

void destroy(ppPilha stack) {
  if (stack == NULL) {
    return;
  }
  int i;
  void *p_element;

  for (i = 0; i < (*stack)->head; i++) {
    p_element = (*stack)->data + (i * (*stack)->data_size);
    free(p_element);
  }

  free((*stack)->data);
}

void push(pPilha stack, void *element) {
  if (is_full(stack)) {
    printf("Error trying to push on a full stack");
    exit(0);
  }

  move_head_up(stack);
  void *top_element = top(stack);

  memcpy(top_element, element, stack->data_size);
}

void *top(pPilha stack) {
  int offset = stack->head * stack->data_size;
  return stack->data + offset;
}

void *pop(pPilha stack) {
  if (is_empty(stack)) {
    printf("Error trying to pop from an empty stack");
    exit(0);
  }

  void *top_element = top(stack);
  move_head_down(stack);
  return top_element;
}
