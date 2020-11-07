# the compiler: gcc for C program, define as g++ for C++
CC = gcc

# compiler flags:
#  -g    adds debugging information to the executable file
#  -Wall turns on most, but not all, compiler warnings
CFLAGS  = -g -Wall 

MIDDLE_FLAGS = -fopenmp

SFLAGS = -lm

# the build target executable:
TARGET = for
OUTPUT = program

all: $(TARGET)

$(TARGET): $(TARGET).c
	$(CC) $(CFLAGS) -o $(OUTPUT) $(TARGET).c $(MIDDLE_FLAGS) $(SFLAGS)

clean:
	$(RM) $(TARGET)
	$(RM) $(OUTPUT)