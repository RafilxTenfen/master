#include <math.h>
#include <omp.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// is_prime return 0 if it's prime
int is_prime(unsigned long number) {
  if (number <= 1) {
    return 1;
  }
  int i;

  for (i = 2; i < number; i++) {
    if (number % i == 0) {
      return 1;
    }
  }

  return 0;
}

// my_pow elevate the base to the power
unsigned long my_pow(unsigned long base, unsigned long power) {
  unsigned long result = 1;
  unsigned long i;

  #pragma omp parallel for reduction(*: result)
  for (i = 0; i < power; i++) {
    result = result * base;
  }

  return result;
}

// allocate_perfect_numbers allocate memory for unsigned long
unsigned long *allocate_perfect_numbers(int qnt) {
  unsigned long *perfect_numbers = NULL;
  perfect_numbers = (unsigned long *)malloc(sizeof(unsigned long) * qnt);

  return perfect_numbers;
}

// generate_perfect_numbers_bf uses brute force to find perfect numbers
unsigned long *generate_perfect_numbers_bf(int qnt) {
  unsigned long *perfect_numbers = allocate_perfect_numbers(qnt);
  int count = 0;
  unsigned long number, divisor, sum_divisors;

  for (number = 1; count < qnt; number++) {
    sum_divisors = 0;

    # pragma omp parallel for default(shared) reduction(+:sum_divisors) private(divisor)
    for (divisor = 1; divisor < number; divisor++) {
      if (number % divisor == 0) {
        sum_divisors += divisor;
      }
    }

    if (sum_divisors == number) {
      perfect_numbers[count] = number;
      count++;
    }
  }

  return perfect_numbers;
}

// generate_perfect_numbers_prime uses euclid form to find perfect numbers
unsigned long *generate_perfect_numbers_prime(int qnt) {
  unsigned long *perfect_numbers = allocate_perfect_numbers(qnt);

  int count = 0;
  for (unsigned long number = 2; count < qnt; number++) {
    if (is_prime(number) == 0) {
      unsigned long mersenne_number = my_pow(2, number) - 1;
      if (is_prime(mersenne_number) == 0) {
        unsigned long perfect_number = my_pow(2, number - 1) * mersenne_number;
        perfect_numbers[count] = perfect_number;
        count++;
      }
    }
  }

  return perfect_numbers;
}

// print_perfect_numbers just print the received perfect_numbers
void print_perfect_numbers(unsigned long *perfect_numbers, int qnt) {
  printf("Perfect Numbers: ");
  for (int i = 0; i < qnt; i++) {
    printf("%ld ", perfect_numbers[i]);
  }
}

// run_perfect_number_brute_force run the brute force startegy to find the perfect numbers
// and calculates the time it took to find those numbers
void run_perfect_number_brute_force(int qnt) {
  clock_t t = clock();
  unsigned long *generated_numbers = generate_perfect_numbers_bf(qnt);
  t = clock() - t;

  print_perfect_numbers(generated_numbers, qnt);

  double time_taken = ((double)t) / CLOCKS_PER_SEC;
  printf("\nIt took %f seconds to find %d perfect numbers using brute force\n", time_taken, qnt);
}

// run_perfect_number_prime_number run the euclid startegy to find the perfect numbers
// and calculates the time it took to find those numbers
void run_perfect_number_prime_number(int qnt) {
  clock_t t = clock();
  unsigned long *generated_numbers = generate_perfect_numbers_prime(qnt);
  t = clock() - t;

  print_perfect_numbers(generated_numbers, qnt);

  double time_taken = ((double)t) / CLOCKS_PER_SEC;
  printf("\nIt took %f seconds to find %d perfect numbers using Euclid \n", time_taken, qnt);
}

int main(int argc, char **agrv) {
  if (argc < 2 && argc > 4) {
    printf("Error, Wrong number of paramerters, Expected: 2 or 3, Received: %d ", argc);
    return 1;
  }

  printf ("Perfect Number Generator C/OpenMP version\n" );
  printf ("\n" );
  printf ("Number of processors available = %d\n", omp_get_num_procs ( ) );
  printf ("Number of threads =              %d\n", omp_get_max_threads ( ) );
  printf ("\n" );

  int qnt = atoi(agrv[1]);
  printf("Perfect numbers to be generated: %d\n", qnt);

  if (argc == 3) {
    char *func = agrv[2];
    if (strcmp(func, "bf") == 0) {  // it can be implemented more ways to generate perfect numbers
      run_perfect_number_brute_force(qnt);
    }
  } else {
    run_perfect_number_prime_number(qnt);
  }

  return 0;
}
