# the compiler: gcc for C program, define as g++ for C++
CC = gcc

MPI = mpicc

# compiler flags:
#  -g    adds debugging information to the executable file
#  -Wall turns on most, but not all, compiler warnings
CFLAGS  = -g -Wall

PRAGMAFLAGS = -fopenmp

SFLAGS = -lm

# the build target executable:
TARGET = main_openmp
TARGET_MPI = main_mpi
TARGET_SEQUEN = main_sequen
SEQUENC = sequen
OUTPUT = generate_openmp
OUTPUT_MPI = generate_mpi
OUTPUT_SEQUEN = generate_sequen

all: $(TARGET)

$(TARGET): $(TARGET).c
	$(CC) $(CFLAGS) $(PRAGMAFLAGS) -o $(OUTPUT) $(TARGET).c $(SFLAGS)
	$(CC) $(CFLAGS) $(PRAGMAFLAGS) -o $(OUTPUT_SEQUEN) $(TARGET_SEQUEN).c $(SFLAGS)
	$(MPI) $(CFLAGS) $(PRAGMAFLAGS) -o $(OUTPUT_MPI) $(TARGET_MPI).c $(SFLAGS)

clean:
	$(RM) $(OUTPUT)
	$(RM) $(OUTPUT_MPI)