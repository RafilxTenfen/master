#include <stdio.h>
#include <omp.h>

int main(int argc, char** agrv) {

  int tid;
  int valor = 3;
  #pragma omp parallel firstprivate(valor) private(tid) 
  {
    tid = omp_get_thread_num();
    printf("Sou a thread %d ... %d \n", tid, valor);
  }


  // escopo
  int tiesc = 999;
  float sum, private_sum;

  #pragma omp parallel private (private_sum) firstprivate(tiesc)
  {
    tiesc = omp_get_thread_num(); //tiesc = 999 copia de valores pelo first private
    private_sum = tid;
    sum = private_sum;
    printf("\n Private Sum(%d) other sum %d", private_sum, sum);
  }

  return 0;
}
// firstprivate passa por parametro com valor ja setado
// private usa essa pariavel pra cada thread