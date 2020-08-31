#include "matriz.h"

#include <stdio.h>
#include <stdlib.h>

matriz_t *matriz_criar(int linhas, int colunas) {
    matriz_t *m = NULL;
    double *bloco = NULL;
    int i;

    m = (matriz_t *)malloc(sizeof(matriz_t));

    m->linhas = linhas;
    m->colunas = colunas;

    m->dados = (double **)malloc(sizeof(double *) * m->linhas);

    bloco = (double *)malloc(sizeof(double) * m->linhas * m->colunas);

    for (i = 0; i < m->linhas; i++) {
        m->dados[i] = &bloco[i * m->colunas];
        // pega enderco do bloco na posicao 1 * 3
    }

    return m;
}

void matriz_destruir(matriz_t *m) {
    if (m == NULL) {
        return;
    }

    free(m->dados[0]);
    free(m->dados);
    free(m);
    return;
}

void matriz_preencher_rand(matriz_t *m) {
    int i, j;
    for (i = 0; i < m->linhas; i++) {
        for (j = 0; j < m->colunas; j++) {
            m->dados[i][j] = random();
        }
    }
}

void matriz_preencher(matriz_t *m, double valor) {
    int i, j;
    for (i = 0; i < m->linhas; i++) {
        for (j = 0; j < m->colunas; j++) {
            m->dados[i][j] = valor;
        }
    }
}

matriz_t *matriz_multiplicar(matriz_t *A, matriz_t *B) {
    matriz_t *m = matriz_criar(A->linhas, A->colunas);

    int i, j, k;
    for (i = 0; i < A->linhas; i++) {
        for (j = 0; j < A->colunas; j++) {
            for (k = 0; k < A->colunas; k++) {
                m->dados[i][j] += A->dados[i][k] * B->dados[k][j];
            }
        }
    }

    return m;
}

void matriz_imprimir(matriz_t *m) {
    if (m == NULL) {
        return;
    }
    int i, j;
    for (i = 0; i < m->linhas; i++) {
        for (j = 0; j < m->colunas; j++) {
            printf("%.2f ", m->dados[i][j]);
        }
        printf("\n");
    }
}

matriz_t *matriz_somar(matriz_t *A, matriz_t *B) {
    matriz_t *m = matriz_criar(A->linhas, A->colunas);

    int i, j;
    for (i = 0; i < A->linhas; i++) {
        for (j = 0; j < A->colunas; j++) {
            m->dados[i][j] = A->dados[i][j] + B->dados[i][j];
        }
    }

    return m;
}
