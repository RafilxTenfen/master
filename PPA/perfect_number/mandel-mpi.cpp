#include <mpi.h>
#include <stdio.h>
#include <stdlib.h>

#include <complex>

using namespace std;

void mandelbrot(char **mat, int sline, int eline, int max_row, int max_column, int max_n, int desloc) {
  int r, c, n;
  int i = desloc;
  int j = 0;
  for (r = sline; r < eline; ++r) {
    j = 0;
    for (c = 0; c < max_column; ++c) {
      complex<float> z;
      n = 0;
      while (abs(z) < 2 && ++n < max_n)
        z = pow(z, 2) + decltype(z)((float)c * 2 / max_column - 1.5,
                                    (float)r * 2 / max_row - 1);
      mat[i][j] = (n == max_n ? '#' : '.');
      j++;
    }
    i++;
  }
}

void print(char **mat, int max_row, int max_column) {
  int r, c;
  for (r = 0; r < max_row; ++r) {
    for (c = 0; c < max_column; ++c) {
      printf("%c", mat[r][c]);
    }
    printf("\n");
  }
}

int main(int argc, char *argv[]) {
  int max_row, max_column, max_n;

  int rank, size;

  if (argc != 4) {
    printf("Usage: %s max_row max_column max_n\n", argv[0]);
    exit(EXIT_FAILURE);
  }

  MPI_Init(&argc, &argv);

  max_row = atoi(argv[1]);
  max_column = atoi(argv[2]);
  max_n = atoi(argv[3]);

  MPI_Comm_rank(MPI_COMM_WORLD, &rank);
  MPI_Comm_size(MPI_COMM_WORLD, &size);

  char **mat_final = NULL;
  char **mat_worker = NULL;
  char *tmp_final = NULL;
  char *tmp_worker = NULL;

  int sline, eline, chunk;

  chunk = max_row / size;
  sline = rank * chunk;
  eline = sline + chunk;

  printf("Rank %d: lines %d to %d, chunk = %d\n", rank, sline, eline - 1,
         chunk);

  if (rank == 0) {
    mat_final = (char **)malloc(sizeof(char *) * max_row);
    tmp_final = (char *)malloc(sizeof(char) * max_row * max_column);
    for (int i = 0; i < max_row; i++) {
      mat_final[i] = &tmp_final[i * max_column];
    }
  }
  mat_worker = (char **)malloc(sizeof(char *) * chunk);
  tmp_worker = (char *)malloc(sizeof(char) * chunk * max_column);
  for (int i = 0; i < chunk; i++) {
    mat_worker[i] = &tmp_worker[i * max_column];
  }

  mandelbrot(mat_worker, sline, eline, max_row, max_column, max_n, 0);

  if ((rank == 0) && (max_row % size)) {
    sline = max_row - (max_row % size);
    eline = max_row;
    printf("Rank %d: lines %d to %d\n", rank, sline, eline - 1);
    mandelbrot(mat_final, sline, eline, max_row, max_column, max_n, sline);
  }

  MPI_Gather(tmp_worker, chunk * max_column, MPI_CHAR, tmp_final,
             chunk * max_column, MPI_CHAR, 0, MPI_COMM_WORLD);

  //if (rank == 0)
  // {
  //print (mat_final, max_row, max_column);
  //}

  MPI_Finalize();
  return 0;
}
