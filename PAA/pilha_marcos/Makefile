# the compiler: gcc for C program, define as g++ for C++
CC = gcc

# compiler flags:
#  -g    adds debugging information to the executable file
#  -Wall turns on most, but not all, compiler warnings
CFLAGS  = -g -Wall -O3

default: main

pilha.o: Pilha.c Pilha_interface.h Pilha_privado.h
	$(CC) $(CFLAGS) -c Pilha.c -o pilha.o

main: main.c pilha.o
	$(CC) $(CFLAGS) main.c pilha.o -o main

clean:
	rm -f main *.o core a.out *.*~ Makefile~

all: main