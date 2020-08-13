#include <stdio.h>
#include <stdlib.h>

// substituicao de texto... # troque todos os N por 10
#define N 10

int main(int argc, char **argv) {
    int *vet_i;  // * = ponteiro para int
    //char * vet_c; ponteiro para char
    // o que e ponteiro = uma variavel que armazena um endereco de memoria para um determinado tipo
    // void * ponteiro genérico
    // tamanho da palavra CPU/SO = 64 = 8 bytes
    int i;  // apenas 1 inteirp = 4 bytes

    // int vetor[10];  //[0] ... [9]

    // vet_i <-- endereco (ABC200)
    // ABC200 [0][][]...[9] ou seja, 40 bytes

    // man malloc
    // void *malloc()

    //sizeof(int) --> 4 bytes * N (10) = 40 bytes
    vet_i = (int *)malloc(sizeof(int) * N);
    // preparacao/alocacao da memoria
    // alocacao dinamica de memoria

    for (i = 0; i < N; i++) {
        vet_i[i] = i * N;
        printf("(for) vet_i[%d]%d\n", i, vet_i[i]);
    }

    return 0;
}