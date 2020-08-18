#include <stdio.h>
#include <stdlib.h>

int main(int argc, char **argv) {
    int i = 10;
    int *p = NULL;
    int **pp = NULL;
    int ***ppp = NULL;

    p = &i;

    pp = &p;

    ppp = &pp;

    printf("inteiro = %d\n", ***ppp);

    return 0;
}