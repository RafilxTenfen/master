#include <stdio.h>
#include <stdlib.h>
#include <omp.h>
#include <time.h>
#define TAM 5

void selectionSort(int v[], int n){
  int i, j, x, aux;

  for (i = 0; i < n; i++){
    x = i;
    for (j = i+1; j < n; j++){
      if( v[j] < v[x] )
        x = j;
    }
    aux = v[i];
    v[i] = v[x];
    v[x] = aux;
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

  printf("Teste - SelectionSort\n");

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
  //for( i = 0; i < TAM; i++ )
  //{
  //  printf("\nVetor [%d]: ", vetor[i]);
  //}

  //obtendo o tempo em segundos
  time(&segundos_inicial);

  //para converter de segundos para o tempo local
  //utilizamos a função localtime
  data_hora_inicial = localtime(&segundos_inicial);

  printf("\nHora - Inicial......: %d:",data_hora_inicial->tm_hour);//hora
  printf("%d:",data_hora_inicial->tm_min);//minuto
  printf("%d\n",data_hora_inicial->tm_sec);//segundo

  double start = omp_get_wtime();
  selectionSort(vetor, TAM);
  double end = omp_get_wtime();

  //obtendo o tempo em segundos
  time(&segundos_final);


  for( i = 0; i < TAM; i++ )
  {
   printf("\nVetor [%d]: ", vetor[i]);
  }

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

