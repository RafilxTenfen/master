#include <stdio.h>
#include <stdlib.h>

#include "Pilha_privado.h"

const int ERROR = -1, SUCESSO = 0, NAO_EXISTE = 0;
const int TAMANHO_INICIAL = 5, QTD_AUMENTO = 2, QTD_REDUCAO = 2;

/*
 * Cria uma pilha e reserva espaço na memória para armazenar a estrutura
 * Retorna um ponteiro para a estrutura pilha_t criada ou
 * NULL no caso de não poder cria-la
 */
pilha_t *Init(int sizeOfItem) {
  pilha_t *pilha = malloc(sizeof(pilha_t));
  if (pilha == NULL) {
    return NULL;
  }
  pilha->topo = 0;
  pilha->sizeOfItem = sizeOfItem;
  pilha->tamanho = TAMANHO_INICIAL;
  pilha->elementos = malloc((unsigned int)TAMANHO_INICIAL * sizeOfItem);

  if (pilha->elementos == NULL) {
    free(pilha);
    return NULL;
  }

  return pilha;
}

/*
 * Recebe um vetor dinamico de
 * elementos junto com o tamanho que tem atualmente
 * Aumenta ou diminue o tamanho, copia cada
 * elemento para o novo vetor se a alocação for ok, caso contrário devolve -1, indicando erro!
 */
int redimensionar_pilha(void ***elementos, int *tamanho, int redimension, int sizeOfItem) {
  void *auxiliar = realloc(*elementos, (unsigned int)redimension * sizeOfItem);
  if (auxiliar == NULL) {
    return ERROR;
  }
  (*elementos) = auxiliar;
  (*tamanho) = redimension;
  return SUCESSO;
}

/*
 * 1 - os dados que recebe são inteiros, devem ser positivos
 * 2 - retorna verdadeiro caso tenha espaço na pilha
 */
bool tem_espaco(int topo, int tamanho) {
  return topo < tamanho;
}

/*
 * O topo é positivo
 * O item a ser salvo é atribuído na pilha e o topo é aumentado
 */
void guardar_elemento(void **elementos, void **elem_guardar, int *topo) {
  elementos[*topo] = (*elem_guardar);
  (*topo)++;
}

/*
 * Empilhar
 * retorna 0 para sucesso ou -1 caso erro
 */
int Push(pilha_t *pilha, void *elemento) {
  if (pilha == NULL) {
    return ERROR;
  }
  if (pilha->elementos == NULL) {
    return ERROR;
  }

  int estado = SUCESSO;
  if (tem_espaco(pilha->topo, pilha->tamanho)) {
    guardar_elemento(pilha->elementos, &elemento, &(pilha->topo));
  } else {
    int novo_tamanho = pilha->tamanho * QTD_AUMENTO;
    estado = redimensionar_pilha(&(pilha->elementos), &(pilha->tamanho), novo_tamanho, pilha->sizeOfItem);

    if (estado == SUCESSO) {
      guardar_elemento(pilha->elementos, &elemento, &(pilha->topo));
    }
  }
  return estado;
}

/*
 * topo e tamanho devem ser positivos
 * deve retornar true para liberar memória, o tamanho deve ser maior que o inicial
 */
bool pode_liberar_memoria(int topo, int tamanho) {
  return tamanho > TAMANHO_INICIAL && topo < tamanho / QTD_REDUCAO;
}

/*
 * Desempilha
 */
int Pop(pilha_t *pilha) {
  if (pilha == NULL) {
    return ERROR;
  }
  if (pilha->elementos == NULL) {
    return ERROR;
  }
  if (pilha->topo == 0) {
    return ERROR;
  }

  (pilha->topo)--;

  int estado = SUCESSO;
  if (pode_liberar_memoria(pilha->topo, pilha->tamanho)) {
    int novo_tamanho = pilha->tamanho / QTD_REDUCAO;
    estado = redimensionar_pilha(&(pilha->elementos), &(pilha->tamanho), novo_tamanho, pilha->sizeOfItem);
  }

  return estado;
}

/*
 * Verifica se a pilha está vazia
 * Retorna true caso vazia, ou false caso contrário
 * Se a pilha não existir, retorna true
 */
bool IsEmpty(pilha_t *pilha) {
  if (pilha == NULL) {
    return true;
  }
  if (pilha->elementos == NULL) {
    return true;
  }
  return pilha->topo == 0;
}

/*
 * Retorna a quantidade de elementos na pilha, ou o tamanho
 */
int Size(pilha_t *pilha) {
  if (pilha == NULL) {
    return NAO_EXISTE;
  }
  if (pilha->elementos == NULL) {
    return NAO_EXISTE;
  }
  return pilha->topo;
}

/*
 * Retorna o elemento do tpo da pilha
 */
void *Top(pilha_t *pilha) {
  if (pilha == NULL) {
    return NULL;
  }
  if (pilha->elementos == NULL) {
    return NULL;
  }
  if (pilha->topo == 0) {
    return NULL;
  }
  int offset = pilha->topo * pilha->sizeOfItem;
  return pilha->elementos + offset;
}

/*
 * Destroi a pilha e libera a memória
 */
void Destroy(pilha_t *pilha) {
  if (pilha != NULL) {
    free(pilha->elementos);
  }
  free(pilha);
}
