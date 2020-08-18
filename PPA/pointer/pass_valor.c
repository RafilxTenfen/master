#include <stdio.h>
#include <stdlib.h>

void pass_valor(int *par1, float *par2) {
    *par1 = 10;
    *par2 = 10.10;
}

int main(int argc, char **argv) {
    int p1 = 5;
    float p2 = 5.1;

    pass_valor(&p1, &p2);

    printf("p1 = %d\np2 = %f\n", p1, p2);

    return 0;
}