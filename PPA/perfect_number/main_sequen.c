#include <math.h>
#include <omp.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LIMIT_PERFECT_NUMBERS 20

// is_prime return 0 if it's prime
int is_prime(unsigned long number) {
  int i, limit;

  limit = sqrt(number);
  for (i = 3; i <= limit; i++) {
    if (number % i == 0) {
      return 1;
    }
  }

  return 0;
}

// my_pow elevate the base to the power
unsigned long my_pow(unsigned long base, unsigned long power) {
  unsigned long result = 1;
  for (int i = 0; i < power; i++) {
    result = result * base;
  }

  return result;
}

// allocate_perfect_numbers allocate memory for unsigned long
unsigned long *allocate_perfect_numbers() {
  unsigned long *perfect_numbers = NULL;
  perfect_numbers = (unsigned long *)malloc(sizeof(unsigned long) * LIMIT_PERFECT_NUMBERS);

  return perfect_numbers;
}


// generate_perfect_numbers_bf uses brute force to find perfect numbers
int generate_perfect_numbers_bf(unsigned long *perfect_numbers, unsigned long limit) {
  int count = 0;
  unsigned long number, divisor;

  for (number = 1; number <= limit; number++) {
    unsigned long sum_divisors = 0;

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

  return count;
}

// generate_perfect_numbers_prime uses euclid form to find perfect numbers
int generate_perfect_numbers_prime(unsigned long *perfect_numbers, unsigned long limit) {
  int count = 0;
  for (unsigned long number = 2; number <= limit; number++) {
    if (is_prime(number) == 0) {
      unsigned long prev_mersenne_number = my_pow(2, number - 1);
      unsigned long mersenne_number = (prev_mersenne_number * 2) - 1;

      if (is_prime(mersenne_number) == 0) {
        unsigned long perfect_number = prev_mersenne_number * mersenne_number;
        perfect_numbers[count] = perfect_number;
        count++;
      }
    }
  }

  return count;
}

// print_perfect_numbers just print the received perfect_numbers
void print_perfect_numbers(unsigned long *perfect_numbers, int qnt) {
  printf("Perfect Numbers: ");
  for (int i = 0; i < qnt; i++) {
    unsigned long number = perfect_numbers[i];
    if (number == 0) {
      break;
    }
    printf("%ld ", number);
  }
}

// run_perfect_number_brute_force run the brute force startegy to find the perfect numbers
// and calculates the time it took to find those numbers
void run_perfect_number_brute_force(int limit) {
  unsigned long *perfect_numbers = allocate_perfect_numbers();

  double start = omp_get_wtime();
  int count = generate_perfect_numbers_bf(perfect_numbers, limit);
  double end = omp_get_wtime();

  print_perfect_numbers(perfect_numbers, count);
  printf("\nIt took %f seconds to find %d perfect numbers using brute force\n", end - start, count);
}

// solve_bhaskara for the equasion t² - t - (limit*2)
unsigned long solve_bhaskara(unsigned long limit) {
  unsigned long delta = 1 - (4 * 1 * (-(limit*2)));
  return 1 + sqrt(delta) / 2;
}

// run_perfect_number_prime_number run the euclid startegy to find the perfect numbers
// and calculates the time it took to find those numbers
void run_perfect_number_prime_number(unsigned long limit) {
  unsigned long *perfect_numbers = allocate_perfect_numbers();

  // limit should be divided by the equasion (2^(x-1)) . (2^x - 1) <= limit, when simplify the equasion it will be log2(positive x bhaskara(limit))
  limit = log2(solve_bhaskara(limit));

  double start = omp_get_wtime();
  int count = generate_perfect_numbers_prime(perfect_numbers, limit);
  double end = omp_get_wtime();

  print_perfect_numbers(perfect_numbers, limit);
  printf("\nIt took %f seconds to find %d perfect numbers using Euclid \n",  end - start, count);
}

int main(int argc, char** agrv) {
  if (argc < 2 && argc > 4) {
    printf("Error, Wrong number of paramerters, Expected: 2 or 3, Received: %d ", argc);
    return 1;
  }

  unsigned long limit = atol(agrv[1]);
  printf("Max Limit %ld to check for Perfect numbers\n", limit);

  if (argc == 3) {
    char* func = agrv[2];
    if (strcmp(func, "bf") == 0) {  // it can be implemented more ways to generate perfect numbers
      run_perfect_number_brute_force(limit);
    }
  } else {
    run_perfect_number_prime_number(limit);
  }

  return 0;
}