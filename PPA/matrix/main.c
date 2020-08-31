#include <stdio.h>
#include <stdlib.h>

#include "matriz.h"

int main(int argc, char **argv) {
    int linhas = 0;
    int colunas = 0;
    matriz_t *A = NULL;
    matriz_t *B = NULL;
    matriz_t *C = NULL;
    matriz_t *D = NULL;

    if ((argc != 2)) {
        printf("Uso: %s <N>\n", argv[0]);
        exit(EXIT_FAILURE);
    }

    linhas = atoi(argv[1]);
    colunas = linhas;

    A = matriz_criar(linhas, colunas);
    B = matriz_criar(linhas, colunas);

    matriz_preencher_rand(A);
    matriz_preencher_rand(B);
    // matriz_preencher(A, 6);
    // matriz_preencher(B, 7);

    printf("Matriz A\n");
    matriz_imprimir(A);

    printf("Matriz B\n");
    matriz_imprimir(B);

    C = matriz_somar(A, B);

    printf("Matriz C - Somar\n");
    matriz_imprimir(C);

    D = matriz_multiplicar(A, B);

    printf("Matriz D - Multiplicar\n");
    matriz_imprimir(D);

    matriz_destruir(A);
    matriz_destruir(B);
    matriz_destruir(C);
    matriz_destruir(D);

    return EXIT_SUCCESS;
}
