#include <stdio.h>
#include <stdlib.h>

// void pass_valor(int *par1, float *par2) {
//     *par1 = 10;
//     *par2 = 10.10;
// }

int fe(char *nome) {
    char *p3;

    p3 = nome;
    printf("(e) %c \n", *p3);
    return 0;
}

int funcA(int *par1) {
    printf("(2a) %d %p \n", *par1, par1);
    par1++;
    printf("(3a) %d %p \n", *par1, par1);
    return 0;
}

int funcB(int par1) {
    printf("(2b) %d %p \n", par1, &par1);
    par1++;
    printf("(3b) %d %p \n", par1, &par1);
    return 0;
}

int fB(float *temp, float *p2) {
    p2 = temp;
    *p2 = 29.0;
    printf("(b2) %.1f \n", *temp);
    return 0;
}

// char *fF(char *p3) {
//     p3 = p3 + 5;
//     printf("(f) %c \n", *p3);
//     return p3;
// }

int fA(int valor, int *p1) {
    valor = 10;
    p1 = &valor;
    *p1 = 20;
    printf("(a2) %d \n", valor);
    return 0;
}

int fD(char aux, char *nome) {
    char *p3;

    p3 = &nome[4];
    aux = *p3;
    printf("(d) %c \n", aux);
    return 0;
}

int fI(int *p5, int *p4) {
    int idade;

    p5 = p4 + 2;
    idade = *p5;
    printf("(i) %d \n", idade);
    return 0;
}

int fJ(int *p5, int idade) {
    int *p4;

    p4 = p5 - 1;
    idade = *p4;
    printf("(j) %d \n", idade);
    return 0;
}

int fK(int *p4, int idade) {
    p4 = p4 - 2;
    idade = *p4;
    printf("(k) %d \n", idade);
    return 0;
}

int fL(int *p5, int *vetor) {
    p5 = &vetor[2] - 1;
    printf("(l) %d \n", *p5);
    return 0;
}

int fM(int *p5) {
    p5++;
    printf("(m) %d \n", *p5);
    return 0;
}

int fN(int *p5) {
    p5++;
    printf("(n) %d \n", *p5);
    return 0;
}

char *fF(char *p3) {
    p3 = p3 + 5;
    printf("(f) %c \n", *p3);
    return p3;
}

int fG(char *p3) {
    char *aux = NULL;
    p3--;
    aux = p3;
    p3++;
    printf("(g) %c \n", *aux);
    return 0;
}

int *fH(int *vetor, int *p4) {
    int idade = 45;

    vetor[0] = 31;
    vetor[1] = idade;
    vetor[2] = 27;
    p4 = ++vetor;
    printf("(h2) %d \n", *p4);
    printf("(h3) %p %p \n", p4, vetor);
    return p4;
}

// int funcA(int **par1) {
//     (*par1)[0] = 100;
//     (*par1)[1] = 200;
//     (*par1)[3] = 300;
//     printf("(funcA) vetor[0] %d %p\n", *(*par1), (*par1));
//     return 0;
// }

// int funcB(int par1) {
//     par1 = 55;
//     printf("(funcB) vetor[0] %d %p\n", par1, &par1);
//     return 0;
// }

// int funcC(int *par1) {
//     *par1 = 555;
//     printf("(funcC) vetor[0] %d %p\n", *par1, par1);
//     return 0;
// }

// int funcD(int **par1) {
//     ++(*par1);
//     printf("(funcD) vetor[0] %d %p\n", *(*par1), (*par1));
//     return 0;
// }

// int quest20(char *p3, char *nome) {
//     char aux;

//     p3 = &nome[0];
//     aux = *p3;
//     printf("(c) %c \n", aux);
//     return 0;
// }

int main(int argc, char **argv) {
    // int i = 7;
    // int *ptr_i = NULL;

    // ptr_i = &i;
    // i++;

    // printf("valor de i = %d\n", i);
    // printf("valor de i = %d\n", *ptr_i);

    // // set a value using pointer
    // *ptr_i = 10;

    // printf("valor de i = %d\n", i);
    // printf("valor de i = %d\n", *ptr_i);

    // int p1 = 5;
    // float p2 = 5.1;

    // pass_valor(&p1, &p2);

    // printf("p1 = %d\np2 = %f\n", p1, p2);

    // QUEST 1 --
    // int i = 5;
    // int *p;

    // p = &i;
    // printf("%p %d %d %d %d \n", p, *p + 2, **&p, 3 * *p, **&p + 4);

    // QUEST 2
    // char *nome = "Algoritmos";

    // fe(nome);

    // QUEST 3
    // char aux;
    // char *nome = "Algoritmos";
    // char *p3;

    // p3 = &nome[0];
    // aux = *p3;
    // printf("%c", aux);

    // p3 = &nome[4];
    // aux = *p3;
    // printf("%c", aux);

    // p3 = nome;
    // printf("%c", *p3);

    // p3 = p3 + 4;
    // printf("%c", *p3);

    // p3 = p3 - 4;
    // printf("%c", *p3);

    // QUEST 4
    // int valor;
    // int *p1;

    // valor = 10;
    // p1 = &valor;
    // *p1 = 20;
    // printf("(a) %d \n", valor);

    // QUEST 5
    // int *ptr_valor = NULL;
    // int valor[3] = {10, 20, 30};

    // ptr_valor = valor;
    // printf("(1) %d %p %p \n", *ptr_valor, &valor[0], &valor[1]);
    // funcA(ptr_valor);
    // funcB(*ptr_valor);
    // printf("(4) %d %p %p \n", *ptr_valor, &valor[0], &valor[1]);

    // QUEST 6
    // float temp;
    // float *p2;

    // temp = 26.5;
    // p2 = &temp;
    // *p2 = 29.0;
    // printf("(b) %.1f \n", temp);

    // QUEST 7
    // float temp = 26.5;
    // float *p2 = NULL;

    // printf("(b1) %.1f \n", temp);
    // fB(&temp, p2);
    // printf("(b3) %.1f \n", temp);

    // QUEST 8
    // char *nome = "Algoritmos";
    // char *p3 = NULL;
    // p3 = fF(nome);
    // printf("quest 8 char %c", *p3);

    // QUEST 9
    // int idade;
    // int vetor[3];
    // int *p4;
    // int *p5;

    // vetor[0] = 31;
    // vetor[1] = 45;
    // vetor[2] = 27;
    // p4 = vetor;
    // idade = *p4;
    // printf("(h) %d \n", idade);

    // p5 = p4 + 1;
    // idade = *p5;
    // printf("(i) %d \n", idade);

    // p4 = p5 + 1;
    // idade = *p4;
    // printf("(j) %d \n", idade);

    // p4 = p4 - 2;
    // idade = *p4;
    // printf("(l) %d \n", idade);

    // p5 = &vetor[2] - 1;
    // printf("(m) %d \n", *p5);
    // p5++;
    // printf("(n) %d \n", *p5);

    // QUEST 10
    // int valor = 5;
    // int *p1 = NULL;

    // printf("(a1) %d \n", valor);
    // fA(valor, p1);
    // printf("(a3) %d \n", valor);

    // int valor = 5;
    // int *p1 = NULL;
    // float temp = 26.5;
    // float *p2 = NULL;
    // char *nome = "Algoritmos";
    // char *p3 = NULL;
    // char aux = 'c';
    // int idade = 10;
    // int vetor[3] = {0, 1, 2};
    // int *p4 = NULL;
    // int *p5 = NULL;
    // fD(aux, nome);

    // QUEST 11
    // int idade = 10;
    // int vetor[3] = {0, 1, 2};
    // int *p4 = NULL;
    // int *p5 = NULL;

    // p4 = p5 = &vetor[1];

    // fI(p5, vetor);
    // fJ(p4, idade);
    // fK(&vetor[2], idade);
    // fL(p5, vetor);
    // fM(p4);
    // fN(p5);

    // QUEST 13
    // int i = 3;
    // int j = 5;
    // int *p = NULL;
    // int *q = NULL;

    // p = &i;
    // q = &j;

    // p = &*&j;
    // i = *&j;
    // printf("i=(*&)j; = %d\n", i);

    // QUEST 14
    // int valor[3] = {10, 20, 30};
    // int *ptr_valor = NULL;

    // ptr_valor = &valor[0];
    // if (*(++ptr_valor) == valor[0])
    //     printf("OK (1)\n");
    // ptr_valor = &valor[0];
    // printf("%d\n", *ptr_valor);
    // if (*(ptr_valor++) == valor[0])
    //     printf("OK (2) \n");

    // QUEST 15
    // int valor = 5;
    // int *p1 = NULL;
    // float temp = 26.5;
    // float *p2 = NULL;
    // char *nome = "Algoritmos";
    // char *p3 = NULL;
    // char aux = 'c';
    // int idade = 10;
    // int vetor[3] = {0, 1, 2};
    // int *p4 = NULL;
    // int *p5 = NULL;

    // p3 = fF(nome);
    // fG(p3);

    // printf("(h1) %p %p \n", p4, vetor);
    // p4 = p5 = fH(vetor, p4);
    // printf("(h5) %d \n", vetor[1]);
    // printf("(h4) %p %p \n", p4, vetor);

    // QUEST 16
    // int i = 3;
    // int j = 5;
    // int *p = NULL;
    // int *q = NULL;

    // p = &i;
    // q = &j;

    // i = (*p) + (++*q);
    // printf("i=(*p)+++*q; = %d \n", i);

    // QUEST 17
    // int *ptr_valor = NULL;
    // int valor[3] = {10, 20, 30};

    // ptr_valor = &valor[0];
    // printf("(main-(a)) vetor [0] %d %p %p \n", *ptr_valor, &valor[0], &valor[1]);
    // funcA(&ptr_valor);
    // printf("(main-(a)) vetor [0] %d %p %p \n", *ptr_valor, &valor[0], &valor[1]);

    // valor[0] = 10;
    // printf("(main-(b)) vetor [0] %d %p %p \n", *ptr_valor, &valor[0], &valor[1]);
    // funcB(valor[0]);
    // printf("(main-(b)) vetor [0] %d %p %p \n", *ptr_valor, &valor[0], &valor[1]);

    // valor[0] = 10;
    // printf("(main-(c)) vetor [0] %d %p %p \n", *ptr_valor, &valor[0], &valor[1]);
    // funcC(valor);
    // printf("(main-(c)) vetor [0] %d %p %p \n", *ptr_valor, &valor[0], &valor[1]);

    // valor[0] = 10;
    // printf("(main-(c)) vetor [0] %d %p %p \n", *ptr_valor, &valor[0], &valor[1]);
    // funcD(&ptr_valor);
    // printf("(main-(c)) vetor [0] %d %p %p \n", *ptr_valor, &valor[0], &valor[1]);

    // QUEST 18
    // int i = 3;
    // int j = 5;
    // int *p = NULL;
    // int *q = NULL;

    // p = &i;
    // q = &j;

    // p = &*&j;
    // printf("%p \n", &p);

    // QUEST 19
    // int i = 30, j = 50;
    // int *p, *q;

    // p = &i;
    // q = &j;

    // printf("%d\n", -4 + **&p);

    // QUEST 20
    // char *nome = "Algoritmos";
    // char *p3 = NULL;

    // quest20(p3, nome);

    return 0;
}
