#include <stdio.h>
#include <stdlib.h>

int main(int argc, char **argv) {
    int i = 7;
    int *ptr_i = NULL;

    ptr_i = &i;
    i++;

    printf("valor de i = %d\n", i);
    printf("valor de i = %d\n", *ptr_i);

    // set a value using pointer
    *ptr_i = 10;

    printf("valor de i = %d\n", i);
    printf("valor de i = %d\n", *ptr_i);

    return 0;
}