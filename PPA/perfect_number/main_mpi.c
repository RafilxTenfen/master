#include <math.h>
#include <omp.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <mpi.h>

#define LIMIT_PERFECT_NUMBERS 20
#define RANK_MASTER 0
#define TAG 0

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
int generate_perfect_numbers_bf(unsigned long *perfect_numbers, unsigned long init, unsigned long end) {
  int count = 0;
  unsigned long number, divisor;

  for (number = init; number <= end; number++) {
    unsigned long sum_divisors = 0;

    for (divisor = 1; divisor < number; divisor++) {
      if (number % divisor == 0) {
        sum_divisors += divisor;
      }
    }

    if (sum_divisors == number) {
      printf("\n generate_perfect_numbers_bf number: %ld", number);
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
void run_perfect_number_brute_force(unsigned long limit) {
  int rank, size;
  double start_time, end_time;
  unsigned long chunk, init, end, rest;

  // MPI_UNSIGNED_LONG
  MPI_Comm_rank(MPI_COMM_WORLD, &rank);
  MPI_Comm_size(MPI_COMM_WORLD, &size);

  chunk = limit / size;
  rest = limit % size;
  init = (chunk * rank) + rest;
  end = init + chunk;

  // printf("\nRank %d Init %ld End %ld\n", rank, init, end);

  start_time = MPI_Wtime();
  if (rank == RANK_MASTER) {
    init = 1; // the master always starts at 1
    unsigned long *master_perfect_numbers = allocate_perfect_numbers();
    int count = generate_perfect_numbers_bf(master_perfect_numbers, init, end);

    // printf("\nMASTER found %d from %ld to %ld => ", count, init, end);
    // print_perfect_numbers(master_perfect_numbers, count);
    // printf("\n ");

    for (int i = 1; i < size; i++) {
      MPI_Status status;
      unsigned long *recv_perfect_numbers = allocate_perfect_numbers();

      MPI_Recv(recv_perfect_numbers, LIMIT_PERFECT_NUMBERS, MPI_UNSIGNED_LONG, MPI_ANY_TAG, TAG, MPI_COMM_WORLD, &status);

      int recv_count;
      MPI_Get_count(&status, MPI_INT, &recv_count);

      //receive the perfect numbers and now add to the list of perfect numbers to be printed
      // printf("\nRANK %d send %d to node MASTER => ", i, recv_count);
      // print_perfect_numbers(recv_perfect_numbers, recv_count);

      recv_count--; // always come with the total count+1 if it's not 0
      for (int j = 0; j < recv_count; j++) {
        unsigned long number = recv_perfect_numbers[j];
        if (number == 0) {
          break;
        }
        master_perfect_numbers[count] = number;
        count++;
      }
    }
    end_time = MPI_Wtime();

    print_perfect_numbers(master_perfect_numbers, count);
    printf("\nIt took %f seconds to find %d perfect numbers using brute force\n", end_time - start_time, count);
  } else {
    unsigned long *send_perfect_numbers = (unsigned long *)malloc(sizeof(unsigned long) * LIMIT_PERFECT_NUMBERS);
    int send_count = generate_perfect_numbers_bf(send_perfect_numbers, init, end);
    // printf("\nRANK %d found %d from %ld to %ld => ", rank, send_count, init, end);
    // print_perfect_numbers(send_perfect_numbers, send_count);
    // printf("\n SEND_COUNT %d", send_count);
    MPI_Send(send_perfect_numbers, send_count, MPI_UNSIGNED_LONG, RANK_MASTER, TAG, MPI_COMM_WORLD);
  }

  MPI_Finalize();
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

  MPI_Init (&argc, &agrv);

  unsigned long limit = atol(agrv[1]);
  // printf("Max Limit %ld to check for Perfect numbers\n", limit);

  if (argc == 3) {
    char* func = agrv[2];
    if (strcmp(func, "bf") == 0) {  // it can be implemented more ways to generate perfect numbers
      run_perfect_number_brute_force(limit);
    }
  } else {
    // run_perfect_number_prime_number(limit);
  }

  return 0;
}

// mpirun --use-hwthread-cpus generate_mpi 10030000 bf
// mpirun --machinefile cluster.txt APP