1 TDA Pilha
═══════════

1.1 Instruções de Uso
─────────────────────

  Disponibilizei um Makefile, que espera um arquivo main.c a ser
  compilado juntamente com os arquivos da pilha para gerar um executável
  main.


1.2 Funções
───────────

  • int is_full(stack_t *stack);
  • int is_empty(stack_t *stack);
  • void *top(stack_t *stack);
  • stack_t init(int data_size);
  • void destroy(stack_t *stack);
  • void push(stack_t *stack, void *element);
  • void *pop(stack_t *stack);


1.3 Manipulando a pilha
───────────────────────

  As funções públicas manipulam um ponteiro para um dado do tipo
  stack_t. A função init() retorna um dado do tipo stack_t cujo endereço
  deve ser passado para outras funções que manipulem essa pilha.


1.4 Checagem por pilha vazia/cheia
──────────────────────────────────

  A interface pública oferece as funções is_empty e is_full para que
  seja possível que o usuário da pilha faça verificações caso deseje.
  Essas funções também são usadas na própria implementação da pilha, de
  forma que um erro informativo seja levantado se o usuário tentar
  utilizar pop em uma pilha fazia ou push em uma pilha cheia.
