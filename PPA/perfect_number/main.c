#include <math.h>
#include <omp.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

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
// unsigned long my_pow(unsigned long base, unsigned long power) {
//   unsigned long result = 1;
//   unsigned long i;

//   #pragma omp for reduction(*: result)
//   for (i = 0; i < power; i++) {
//     result = result * base;
//   }

//   return result;
// }

// allocate_perfect_numbers allocate memory for unsigned long
unsigned long *allocate_perfect_numbers(int qnt) {
  unsigned long *perfect_numbers = NULL;
  perfect_numbers = (unsigned long *)malloc(sizeof(unsigned long) * qnt);

  return perfect_numbers;
}

// generate_perfect_numbers_bf uses brute force to find perfect numbers
unsigned long *generate_perfect_numbers_bf(int limit) {
  unsigned long *perfect_numbers = allocate_perfect_numbers(limit);
  int count = 0;
  unsigned long number, divisor;

  # pragma omp parallel shared(perfect_numbers, count) private(number, divisor)
  {

    # pragma omp for
    for (number = 1; number < 8900; number++) {
      unsigned long sum_divisors = 0;

      for (divisor = 1; divisor < number; divisor++) {
        if (number % divisor == 0) {
          sum_divisors += divisor;
        }
      }

      if (count < limit && sum_divisors == number) {
        perfect_numbers[count] = number;
        count++;
      }

    }

  }

  return perfect_numbers;
}

// # pragma omp single 
// #pragma omp for ordered
// # pragma omp for reduction(+:sum_divisors) 

// generate_perfect_numbers_prime uses euclid form to find perfect numbers
unsigned long *generate_perfect_numbers_prime(int qnt) {
  unsigned long *perfect_numbers = allocate_perfect_numbers(qnt);
  unsigned long number, mersenne_number, prev_mersenne_number, result_pow, pow_i;
  int count = 0;
  
  // # pragma omp parallel shared(perfect_numbers)
  // {
    for (number = 2; count < qnt; number++) {
      if (is_prime(number) == 0) {
      
        // pow of a number
        result_pow = 1;

        # pragma omp parallel for default(shared) private(pow_i) reduction(*:result_pow) 
        for (pow_i = 0; pow_i < number - 1; pow_i++) {
          result_pow = result_pow * 2;
        }

        // end of pow
        prev_mersenne_number = result_pow;
        mersenne_number = (prev_mersenne_number * 2) - 1;

        if (is_prime(mersenne_number) == 0) {
          perfect_numbers[count] = prev_mersenne_number * mersenne_number;
          count++;
        }

      }
    }
  // }

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
void run_perfect_number_brute_force(int limit) {
  clock_t t = clock();
  unsigned long *generated_numbers = generate_perfect_numbers_bf(limit);
  t = clock() - t;

  print_perfect_numbers(generated_numbers, limit);

  double time_taken = ((double)t) / CLOCKS_PER_SEC;
  printf("\nIt took %f seconds to find %d perfect numbers using brute force\n", time_taken, limit);
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
  printf ("Number of processors available =            %d\n", omp_get_num_procs ( ) );
  printf ("Number Max of threads omp_get_max_threads = %d\n", omp_get_max_threads ( ) );
  printf ("\n" );

  int limit = atoi(agrv[1]);
  printf("Perfect numbers to be generated: %d\n", limit);

  if (argc == 3) {
    char *func = agrv[2];
    if (strcmp(func, "bf") == 0) {  // it can be implemented more ways to generate perfect numbers
      run_perfect_number_brute_force(limit);
    }
  } else {
    run_perfect_number_prime_number(limit);
  }

  return 0;
}
