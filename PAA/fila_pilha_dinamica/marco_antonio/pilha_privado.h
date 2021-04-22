typedef struct Pilha {
  void **dados;
  int tamanho_dados;
  int tamanho_atual;
  int tamanho_maximo;
} Pilha;