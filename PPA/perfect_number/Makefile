# the compiler: gcc for C program, define as g++ for C++
CC = gcc

# compiler flags:
#  -g    adds debugging information to the executable file
#  -Wall turns on most, but not all, compiler warnings
CFLAGS  = -g -Wall

PRAGMAFLAGS = -fopenmp

SFLAGS = -lm

# the build target executable:
TARGET = main
SEQUENC = sequen
OUTPUT = generate

all: $(TARGET)

$(TARGET): $(TARGET).c
	$(CC) $(CFLAGS) $(PRAGMAFLAGS) -o $(OUTPUT) $(TARGET).c $(SFLAGS)
	$(CC) $(CFLAGS) $(PRAGMAFLAGS) -o $(OUTPUT)_$(SEQUENC) $(TARGET)_$(SEQUENC).c $(SFLAGS)

clean:
	$(RM) $(TARGET)
	$(RM) $(OUTPUT)