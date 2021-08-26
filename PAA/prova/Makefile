# the compiler: gcc for C program, define as g++ for C++
CC = gcc

# compiler flags:
#  -g    adds debugging information to the executable file
#  -Wall turns on most, but not all, compiler warnings
CFLAGS  = -g -Wall -O3

default: main

# Fila.o: Fila.c Fila_publica.h Fila_privada.h
# 	$(CC) $(CFLAGS) -c Fila.c -o Fila.o

main: main.c
	$(CC) $(CFLAGS) main.c -o main

clean:
	rm -f main *.o core a.out *.*~ Makefile~

run: clean main
	./main

all: main