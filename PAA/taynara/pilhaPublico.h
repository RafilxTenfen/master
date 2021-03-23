#ifndef PILHAPUBLICO_H
#define PILHAPUBLICO_H


struct Pilha {
    int topo;
    int tamanho;
    int tamanhoDados;
    void** dados;
};

typedef struct Pilha pilha;

pilha* criarPilha(int capacidadePilha, int tamanhoDados);

void push(pilha* p, int valor);

void pop(pilha* p);

void consultarTopo(pilha* p);

void mostrarPilha(pilha* p);

int removerPilha(pilha* p);

#endif // PILHAPUBLICO_H
