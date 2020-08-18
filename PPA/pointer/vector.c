#include <stdio.h>
#include <stdlib.h>

int main(int argc, char **argv) {
    int valor[3] = {10, 20, 30};
    int *ptr_valor = NULL;

    printf("valor = %p\n&valor = %p\n&valor[0]=%p\n", valor, &valor, &valor[0]);
    // aponta para o início do vetor

    printf("&valor[1] = %p\n", &valor[1]);

    printf("valor[0] = %d\n", valor[0]);
    ptr_valor = &valor[0];

    printf("valor[0] = %d\n", *ptr_valor);
    ptr_valor++;

    printf("valor[1] = %d\n", *ptr_valor);
    return 0;
}