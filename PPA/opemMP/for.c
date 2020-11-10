#include <omp.h>
#include <stdio.h>
#include <stdlib.h>

int main (int argc, char *argv[]) {
  int i, n = 10000;
  float a[10000], b[10000], sum;

  #pragma omp parallel for
  for (i=0; i < n; i++) {
   a[i] = b[i] = i * 1.0; // não tem dependencia
  }
  sum = 0.0;

  #pragma omp parallel for reduction (+:sum)
  for (i=0; i < n; i++) {
    sum = sum + (a[i] * b[i]);
  }

  printf("   Sum = %f\n", sum);

  return 0;
}