
#include <stdio.h>
#include <stdlib.h>
#include "Stack_Public.h"

int main(int argc, char **argv) {
  pop();
  int element1 = 4;
  push(element1);
  push(6);

  display();


  printf("Pop1 %d", pop());
  printf("Pop2 %d", pop());
  int element2 = pop();
  printf("Element2 %d", element2);
  display();
}