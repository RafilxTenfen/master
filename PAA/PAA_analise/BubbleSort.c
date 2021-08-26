#include <stdio.h>
#include <stdlib.h>
#include <omp.h>
#include <time.h>
#define TAM 100000

void bubbleSort(int v[], int n){
int i, j, aux;

  for (i = n - 1; i > 0; i--){
    for (j = 0; j < i; j++){
      if (v[j] > v[j+1]){
        aux = v[j];
        v[j] = v[j+1];
        v[j+1] = aux;
      }
    }
  }
}

int main()
{

  //ponteiro para struct que armazena data e hora
  struct tm *data_hora_inicial;
  struct tm *data_hora_final;

  //variáveis do tipo time_t para armazenar o tempo em segundos
  time_t segundos_inicial;
  time_t segundos_final;

  //vairáveis gerais
  int vetor[TAM];
  int i = 0;
  int x = 0;

  printf("Teste - BubbleSort\n");

  // -------------------------------------------------------------
  // Teste 1
  // Monta um vetor ordenado
  // for( i = 0; i < TAM; i++ )
  // {
  //   vetor[i] = i;
  // }
  // -------------------------------------------------------------


  // -------------------------------------------------------------
  // Teste 2
  // Monta um vetor deordenado
  //printf("intervalo da rand: [0,%d]\n", (RAND_MAX));
  //
  for( i = 0; i < TAM; i++ )
  {
   vetor[i] = rand();
  }
  // -------------------------------------------------------------


  // Mostra o vetor
  // for( i = 0; i < TAM; i++ )
  // {
  //  printf("\nVetor [%d]: ", vetor[i]);
  // }

  //obtendo o tempo em segundos
  time(&segundos_inicial);

  //para converter de segundos para o tempo local
  //utilizamos a função localtime
  data_hora_inicial = localtime(&segundos_inicial);

  printf("\nHora - Inicial......: %d:",data_hora_inicial->tm_hour);//hora
  printf("%d:",data_hora_inicial->tm_min);//minuto
  printf("%d\n",data_hora_inicial->tm_sec);//segundo

  double start = omp_get_wtime();
  bubbleSort(vetor,TAM);
  double end = omp_get_wtime();

  //obtendo o tempo em segundos
  time(&segundos_final);

  //para converter de segundos para o tempo local
  //utilizamos a função localtime
  data_hora_final = localtime(&segundos_final);

  printf("\nHora - Final........: %d:",data_hora_final->tm_hour);//hora
  printf("%d:",data_hora_final->tm_min);//minuto
  printf("%d\n",data_hora_final->tm_sec);//segundo

  printf("\nTempo Decorrido: %ld", (segundos_final - segundos_inicial));
  printf(" segundos\n");

  printf("\nTamanho do Vetor: [%ld]\n",sizeof(vetor)/4);
  printf("\nVetor: %d timelapse: [%f]\n", TAM, end - start);

  return 0;

}

