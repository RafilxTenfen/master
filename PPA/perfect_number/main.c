#include <stdio.h>
#include <stdlib.h>
#include <time.h>

// is_perfect_number "brute force"
// Sum all the divisors of the number.
// If the sum is equal to the number, return 0.
int is_perfect_number_bf(int number) {
  int sum = 0;
  for (int i = 1; sum <= number && i < number; i++) {
    if (number % i == 0) {
      sum += i;
    }
  }

  if (sum != number) {
    return 1;
  }
  return 0;
}

int* generate_perfect_numbers(int qnt) {
  int* perfect_numbers = NULL;
  perfect_numbers = (int*)malloc(sizeof(int) * qnt);

  int count = 0;
  for (int number = 1; count < qnt; number++) {
    if (is_perfect_number_bf(number) == 0) {
      perfect_numbers[count] = number;
      count++;
    }
  }

  return perfect_numbers;
}

void print_perfect_numbers(int* perfect_numbers, int qnt) {
  printf("Perfect Numbers: ");
  for (int i = 0; i < qnt; i++) {
    printf("%d ", perfect_numbers[i]);
  }
}

int main(int argc, char** agrv) {
  if (argc != 2) {
    printf("Error, Wrong number of paramerters, Expected: 2, Received: %d ", argc);
    return 1;
  }

  int qnt = atoi(agrv[1]);
  printf("Perfect numbers to be generated: %d\n", qnt);

  clock_t t = clock();
  int* generated_numbers = generate_perfect_numbers(qnt);
  t = clock() - t;

  print_perfect_numbers(generated_numbers, qnt);

  double time_taken = ((double)t) / CLOCKS_PER_SEC;
  printf("\nIt took %f seconds to find %d perfect numbers \n", time_taken, qnt);

  return 0;
}
