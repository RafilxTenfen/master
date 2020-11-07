#include <complex>
#include <iostream>

using namespace std;

int main() {
  int max_row, max_column, max_n;
  cin >> max_row;
  cin >> max_column;
  cin >> max_n;

  char **mat = (char **)malloc(sizeof(char *) * max_row);

  for (int i = 0; i < max_row; i++) {
    mat[i] = (char *)malloc(sizeof(char) * max_column);
  }

  // #pragma omp parallel shared(mat, max_row, max_column, max_n)
  //   {
  // #pragma omp for schedule(dynamic)
  for (int r = 0; r < max_row; ++r) {
    for (int c = 0; c < max_column; ++c) {
      //para cada celula da matriz
      complex<float> z;
      int n = 0;
      while (abs(z) < 2 && ++n < max_n)
        z = (z * z) + decltype(z)(
                          (float)c * 2 / max_column - 1.5,
                          (float)r * 2 / max_row - 1);
      mat[r][c] = (n == max_n ? '#' : '.');
    }
  }
  // }  // end

  for (int r = 0; r < max_row; ++r) {
    for (int c = 0; c < max_column; ++c)
      std::cout << mat[r][c];
    cout << '\n';
  }
}

/*
________________________________________________________ WITHOUT OPEN MP
Executed in  111,15 secs   fish           external 
   usr time   49,87 secs  560,00 micros   49,87 secs 
   sys time    0,34 secs  223,00 micros    0,34 secs 

Executed in   83,76 secs   fish           external GUIDED
   usr time   56,89 secs  630,00 micros   56,89 secs 
   sys time    0,38 secs  253,00 micros    0,38 secs 

Executed in   53,07 secs   fish           external DYNAMIC
   usr time   55,37 secs  862,00 micros   55,37 secs 
   sys time    0,35 secs  352,00 micros    0,35 secs 

*/